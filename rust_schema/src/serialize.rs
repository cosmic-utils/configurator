use std::{any::Any, collections::BTreeMap};

use facet::{Facet, Field, ScalarType, StructKind, Type, UserType};
use facet_format::{FormatSerializer, ScalarValue, SerializeError};
use facet_reflect::Peek;

use crate::{
    Number, Value,
    number::{F32, F64},
};

#[derive(Debug)]
pub struct ToValueError {
    msg: String,
}

impl ToValueError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }
}

impl core::fmt::Display for ToValueError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl core::error::Error for ToValueError {}

struct ValueSerializer {
    stack: Vec<StackFrame>,
    result: Option<Value>,
    pending: Option<Pending>,
}

enum Pending {
    Struct { name: String },
    UnitStruct { name: String },
    TupleStruct { name: String },
    Tuple { field_count: usize },
    EnumVariantTuple { name: String },
    EnumVariantStruct { name: String },
}

#[derive(Debug)]
enum StackFrame {
    UnitStruct {
        name: String,
    },
    TupleStruct {
        name: String,
        elems: Vec<Value>,
    },
    Struct {
        name: String,
        elems: BTreeMap<String, Value>,
        pending_key: Option<String>,
    },
    Array {
        elems: Vec<Value>,
    },
    Tuple {
        elems: Vec<Value>,
    },
    Map {
        elems: BTreeMap<String, Value>,
        pending_key: Option<String>,
    },
    EnumVariantUnit,
    EnumVariantTuple {
        name: String,
        elems: Vec<Value>,
    },
    EnumVariantStruct {
        name: String,
        elems: BTreeMap<String, Value>,
        pending_key: Option<String>,
    },
}

impl ValueSerializer {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            result: None,
            pending: None,
        }
    }

    fn finish(self) -> Value {
        self.result.unwrap()
    }

    fn emit(&mut self, value: Value) {
        // println!("emit: {:?} : {:?}", self.stack, value);

        match self.stack.last_mut() {
            Some(StackFrame::Struct {
                elems, pending_key, ..
            }) => {
                let key = pending_key
                    .take()
                    .expect("emit called on struct without pending key");

                elems.insert(key, value);
            }

            Some(StackFrame::TupleStruct { elems, .. }) => {
                elems.push(value);
            }
            Some(StackFrame::Array { elems }) => {
                elems.push(value);
            }
            Some(StackFrame::Tuple { elems }) => {
                elems.push(value);
            }

            Some(StackFrame::EnumVariantTuple { elems, .. }) => {
                elems.push(value);
            }

            Some(StackFrame::EnumVariantStruct {
                elems, pending_key, ..
            }) => {
                let key = pending_key
                    .take()
                    .expect("emit called on struct without pending key");

                elems.insert(key, value);
            }

            Some(StackFrame::Map { elems, pending_key }) => match pending_key.take() {
                Some(key) => {
                    elems.insert(key, value);
                }
                None => {
                    match value {
                        Value::String(v) => *pending_key = Some(v),
                        _ => panic!("key of map is not a string"),
                    };
                }
            },
            Some(StackFrame::UnitStruct { .. }) => {
                panic!("emit value in unit struct")
            }
            Some(StackFrame::EnumVariantUnit) => {
                if let Value::String(name) = value {
                    self.stack.pop();
                    self.emit(Value::EnumVariantUnit(name));
                } else {
                    panic!("value of enum variant unit is not a string")
                }
            }
            None => {
                self.result = Some(value);
            }
        }
    }
}

impl FormatSerializer for ValueSerializer {
    type Error = ToValueError;

    fn struct_metadata(&mut self, shape: &facet_core::Shape) -> Result<(), Self::Error> {
        println!("struct_metadata: {shape:?}");
        println!();

        if let Type::User(UserType::Struct(struct_type)) = &shape.ty {
            let pending = match struct_type.kind {
                StructKind::Unit => Pending::UnitStruct {
                    name: shape.type_identifier.to_owned(),
                },
                StructKind::TupleStruct => Pending::TupleStruct {
                    name: shape.type_identifier.to_owned(),
                },
                StructKind::Struct => Pending::Struct {
                    name: shape.type_identifier.to_owned(),
                },
                StructKind::Tuple => unreachable!(),
            };

            self.pending = Some(pending);
        }

        Ok(())
    }

    fn begin_struct(&mut self) -> Result<(), Self::Error> {
        println!("begin_struct");
        println!();

        let stack_frame = match self.pending.take() {
            Some(pending) => match pending {
                Pending::Struct { name } => StackFrame::Struct {
                    name,
                    elems: BTreeMap::new(),
                    pending_key: None,
                },
                Pending::UnitStruct { name } => StackFrame::UnitStruct { name },
                Pending::TupleStruct { name } => StackFrame::TupleStruct {
                    name,
                    elems: Vec::new(),
                },
                Pending::Tuple { field_count } => unreachable!(),
                Pending::EnumVariantTuple { name } => StackFrame::EnumVariantTuple {
                    name,
                    elems: Vec::new(),
                },
                Pending::EnumVariantStruct { name } => StackFrame::EnumVariantStruct {
                    name,
                    elems: BTreeMap::new(),
                    pending_key: None,
                },
            },
            None => panic!("no pending struct"),
        };

        self.stack.push(stack_frame);

        Ok(())
    }

    fn field_metadata(&mut self, field: &facet_reflect::FieldItem) -> Result<(), Self::Error> {
        println!("field_metadata: {field:?}");
        println!();

        if let Some(field) = &field.field
            && let shape = field.shape.get()
            && let Type::User(UserType::Struct(struct_type)) = &shape.ty
        {
            let pending = match struct_type.kind {
                // StructKind::Unit => Pending::UnitStruct {
                //     name: shape.type_identifier.to_owned(),
                // },

                // StructKind::Struct => Pending::Struct {
                //     name: shape.type_identifier.to_owned(),
                // },
                StructKind::TupleStruct => Pending::TupleStruct {
                    name: shape.type_identifier.to_owned(),
                },
                StructKind::Tuple => Pending::Tuple {
                    field_count: struct_type.fields.len(),
                },
                StructKind::Unit => todo!(),
                StructKind::Struct => todo!(),
            };
            self.pending = Some(pending);
        }

        Ok(())
    }

    fn variant_metadata(
        &mut self,
        variant: &'static facet_core::Variant,
    ) -> Result<(), Self::Error> {
        println!("variant_metadata: {variant:?}");
        println!();

        match variant.data.kind {
            StructKind::Unit => {
                self.stack.push(StackFrame::EnumVariantUnit);
            }
            StructKind::TupleStruct => {
                self.pending = Some(Pending::EnumVariantTuple {
                    name: variant.name.to_owned(),
                })
            }
            StructKind::Struct => {
                self.pending = Some(Pending::EnumVariantStruct {
                    name: variant.name.to_owned(),
                })
            }
            StructKind::Tuple => todo!(),
        }

        Ok(())
    }

    fn field_metadata_with_value(
        &mut self,
        _field: &facet_reflect::FieldItem,
        _value: Peek<'_, '_>,
    ) -> Result<bool, Self::Error> {
        println!("field_metadata_with_value: {_field:?}");
        println!();

        Ok(false)
    }

    fn map_encoding(&self) -> facet_format::MapEncoding {
        facet_format::MapEncoding::Pairs
    }

    fn begin_map_with_len(&mut self, _len: usize) -> Result<(), Self::Error> {
        println!("begin_map_with_len");
        println!();

        self.stack.push(StackFrame::Map {
            elems: BTreeMap::new(),
            pending_key: None,
        });

        Ok(())
    }

    fn field_key(&mut self, key: &str) -> Result<(), Self::Error> {
        println!("field_key: {key}");
        println!();

        match self.stack.last_mut() {
            Some(StackFrame::Struct { pending_key, .. }) => {
                *pending_key = Some(key.to_owned());
            }
            Some(StackFrame::Map { pending_key, .. }) => {
                *pending_key = Some(key.to_owned());
            }
            Some(StackFrame::EnumVariantTuple { .. }) => {
                // pass ?
            }
            Some(StackFrame::EnumVariantStruct { .. }) => {
                // pass ?
            }
            _ => panic!("field_key called outside of object"),
        }
        Ok(())
    }

    fn end_struct(&mut self) -> Result<(), Self::Error> {
        println!("end_struct");
        println!();

        match self.stack.pop() {
            Some(StackFrame::Struct { name, elems, .. }) => {
                self.emit(Value::Struct(name, elems));
                Ok(())
            }
            Some(StackFrame::UnitStruct { name }) => {
                self.emit(Value::UnitStruct(name));
                Ok(())
            }
            Some(StackFrame::TupleStruct { name, elems }) => {
                self.emit(Value::TupleStruct(name, elems));
                Ok(())
            }
            Some(StackFrame::EnumVariantTuple { name, elems }) => {
                self.emit(Value::EnumVariantTuple(name, elems));
                Ok(())
            }
            _ => panic!("end_struct called without matching begin_struct"),
        }
    }

    fn end_map(&mut self) -> Result<(), Self::Error> {
        println!("end_map");
        println!();

        match self.stack.pop() {
            Some(StackFrame::Map { elems, .. }) => {
                self.emit(Value::Map(elems));
            }
            _ => panic!("end_struct called without matching begin_struct"),
        }
        Ok(())
    }

    fn begin_seq_with_len(&mut self, len: usize) -> Result<(), Self::Error> {
        println!("begin_seq_with_len");
        println!();

        self.stack.push(StackFrame::Array {
            elems: Vec::with_capacity(len),
        });
        Ok(())
    }

    fn begin_seq(&mut self) -> Result<(), Self::Error> {
        println!("begin_seq");
        println!();

        let stack_frame = if let Some(pending) = self.pending.take() {
            match pending {
                Pending::Struct { name } => unreachable!(),
                Pending::UnitStruct { name } => unreachable!(),
                Pending::TupleStruct { name } => StackFrame::TupleStruct {
                    name,
                    elems: Vec::new(),
                },
                Pending::Tuple { field_count } => StackFrame::Tuple {
                    elems: Vec::with_capacity(field_count),
                },
                Pending::EnumVariantTuple { name } => unreachable!(),
                Pending::EnumVariantStruct { name } => unreachable!(),
            }
        } else {
            StackFrame::Array { elems: Vec::new() }
        };

        self.stack.push(stack_frame);
        Ok(())
    }

    fn end_seq(&mut self) -> Result<(), Self::Error> {
        println!("end_seq");
        println!();

        match self.stack.pop() {
            Some(StackFrame::Array { elems }) => {
                self.emit(Value::Array(elems));
            }
            Some(StackFrame::Tuple { elems }) => {
                self.emit(Value::Tuple(elems));
            }
            Some(StackFrame::TupleStruct { name, elems }) => {
                self.emit(Value::TupleStruct(name, elems));
            }
            _ => panic!("end_seq called without matching begin_seq"),
        }
        Ok(())
    }

    fn typed_scalar(
        &mut self,
        scalar_type: facet::ScalarType,
        value: Peek<'_, '_>,
    ) -> Result<(), Self::Error> {
        println!("typed_scalar");
        println!();

        let value = match scalar_type {
            ScalarType::Unit => Value::Unit,
            ScalarType::Bool => Value::Bool(*value.get::<bool>().unwrap()),
            ScalarType::Char => Value::Char(*value.get::<char>().unwrap()),
            ScalarType::Str | ScalarType::String | ScalarType::CowStr => {
                Value::String(value.as_str().unwrap().to_owned())
            }
            ScalarType::U8 => Value::Number(Number::U8(*value.get::<u8>().unwrap())),
            ScalarType::U16 => Value::Number(Number::U16(*value.get::<u16>().unwrap())),
            ScalarType::U32 => Value::Number(Number::U32(*value.get::<u32>().unwrap())),
            ScalarType::U64 => Value::Number(Number::U64(*value.get::<u64>().unwrap())),
            ScalarType::U128 => Value::Number(Number::U128(*value.get::<u128>().unwrap())),
            ScalarType::USize => Value::Number(Number::USize(*value.get::<usize>().unwrap())),
            ScalarType::I8 => Value::Number(Number::I8(*value.get::<i8>().unwrap())),
            ScalarType::I16 => Value::Number(Number::I16(*value.get::<i16>().unwrap())),
            ScalarType::I32 => Value::Number(Number::I32(*value.get::<i32>().unwrap())),
            ScalarType::I64 => Value::Number(Number::I64(*value.get::<i64>().unwrap())),
            ScalarType::I128 => Value::Number(Number::I128(*value.get::<i128>().unwrap())),
            ScalarType::ISize => Value::Number(Number::ISize(*value.get::<isize>().unwrap())),
            ScalarType::F32 => Value::Number(Number::F32(F32(*value.get::<f32>().unwrap()))),
            ScalarType::F64 => Value::Number(Number::F64(F64(*value.get::<f64>().unwrap()))),
            _ => {
                todo!("{:?}", scalar_type)
            }
        };

        self.emit(value);
        Ok(())
    }

    fn scalar(&mut self, scalar: ScalarValue<'_>) -> Result<(), Self::Error> {
        println!("scalar");
        println!();

        let value = match scalar {
            ScalarValue::Unit => Value::Unit,
            ScalarValue::Null => Value::Null,
            ScalarValue::Bool(v) => Value::Bool(v),
            ScalarValue::Char(v) => Value::Char(v),
            ScalarValue::I64(v) => Value::Number(Number::I64(v)),
            ScalarValue::U64(v) => Value::Number(Number::U64(v)),
            ScalarValue::I128(v) => Value::Number(Number::I128(v)),
            ScalarValue::U128(v) => Value::Number(Number::U128(v)),
            ScalarValue::F64(v) => Value::Number(Number::F64(F64(v))),
            ScalarValue::Str(v) => Value::String(v.into_owned()),
            ScalarValue::Bytes(v) => todo!(),
        };
        self.emit(value);
        Ok(())
    }
}

pub fn to_value<'facet, T: Facet<'facet>>(
    value: &T,
) -> Result<Value, SerializeError<ToValueError>> {
    let mut serializer = ValueSerializer::new();
    facet_format::serialize_root(&mut serializer, Peek::new(value))?;
    Ok(serializer.finish())
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    use facet::Facet;

    use crate::{
        Number, Value,
        number::{F32, F64},
        serialize::to_value,
    };

    #[derive(Facet, Debug)]
    struct SimpleStruct {
        u: (),
        opt: Option<u8>,
        b: bool,
        f: f64,
        i: i32,
        c: char,
        s: String,
        v: Vec<u8>,
        t: (String, String),
        h: HashMap<String, i32>,
    }

    #[derive(Facet, Debug)]
    struct UnitStruct;

    #[derive(Facet, Debug)]
    struct TupleStruct(String, i32);

    #[derive(Facet, Debug)]
    #[repr(u8)]
    enum EnumSimple {
        Unit,
        Tuple(String, i32),
        Struct { b: bool, s: String },
    }

    #[derive(Facet, Debug)]
    struct Complex {
        s: String,
    }

    impl Complex {
        fn new(c: &str) -> Self {
            Self { s: c.into() }
        }
    }

    #[derive(Facet, Debug)]
    struct ComplexNested {
        c: Complex,
        opt_c: Option<Box<ComplexNested>>,
        opt_e: Option<Box<EnumNested>>,
    }

    impl ComplexNested {
        fn new(c: &str, o: Option<ComplexNested>, o2: Option<EnumNested>) -> Self {
            Self {
                c: Complex::new(c),
                opt_c: o.map(Box::new),
                opt_e: o2.map(Box::new),
            }
        }
    }

    #[derive(Facet, Debug)]
    #[repr(u8)]
    enum EnumNested {
        Unit,
        Tuple(String, (Complex, i32)),
        Struct { c: Complex, s: String },
    }

    #[derive(Facet, Debug)]
    struct StructNested {
        v: Vec<ComplexNested>,
        t: (String, ComplexNested),
        m: HashMap<String, ComplexNested>,
    }

    #[test]
    fn struct_() {
        let c = SimpleStruct {
            u: (),
            opt: None,
            b: false,
            f: 3.66,
            i: 4,
            c: '\n',
            s: String::from("hello"),
            v: vec![1, 2, 3],
            t: (String::from("hello"), String::from("world")),
            h: {
                let mut h = HashMap::new();
                h.insert("key1".into(), 6);
                h.insert("key2".into(), 7);
                h
            },
        };

        let value = to_value(&c).unwrap();

        assert_eq!(
            value,
            Value::Struct(
                String::from("SimpleStruct"),
                [
                    ("u", Value::Unit),
                    ("opt", Value::Null),
                    ("b", Value::Bool(false)),
                    ("f", Value::Number(Number::F64(F64(3.66)))),
                    ("i", Value::Number(Number::I32(4))),
                    ("c", Value::Char('\n')),
                    ("s", Value::String(String::from("hello"))),
                    (
                        "v",
                        Value::Array(
                            [1, 2, 3]
                                .map(|v| Value::Number(Number::U8(v)))
                                .into_iter()
                                .collect()
                        )
                    ),
                    (
                        "t",
                        Value::Tuple(vec![
                            Value::String(String::from("hello")),
                            Value::String(String::from("world"))
                        ])
                    ),
                    (
                        "h",
                        Value::Map(
                            [("key1", 6), ("key2", 7)]
                                .map(|(k, v)| (k.to_owned(), Value::Number(Number::I32(v))))
                                .into_iter()
                                .collect()
                        )
                    ),
                ]
                .map(|(k, v)| (k.to_owned(), v))
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn unit_struct() {
        let c = UnitStruct;

        let value = to_value(&c).unwrap();

        assert_eq!(value, Value::UnitStruct(String::from("UnitStruct")))
    }

    #[test]
    #[ignore = "facet limitation ig"]
    fn tuple_struct() {
        let c = TupleStruct("hello".into(), 3);

        let value = to_value(&c).unwrap();

        assert_eq!(
            value,
            Value::TupleStruct(
                String::from("TupleStruct"),
                vec![
                    Value::String(String::from("hello")),
                    Value::Number(Number::I32(3)),
                ]
            )
        )
    }

    #[test]
    fn tuple_struct2() {
        #[derive(Facet, Debug)]
        struct TupleStruct2 {
            t: TupleStruct,
        }

        let c = TupleStruct2 {
            t: TupleStruct("hello".into(), 3),
        };

        let value = to_value(&c).unwrap();

        assert_eq!(
            value,
            Value::Struct(
                String::from("TupleStruct2"),
                [(
                    "t",
                    Value::TupleStruct(
                        String::from("TupleStruct"),
                        vec![
                            Value::String(String::from("hello")),
                            Value::Number(Number::I32(3)),
                        ]
                    )
                )]
                .map(|(k, v)| (k.to_owned(), v))
                .into_iter()
                .collect()
            )
        )
    }

    #[test]
    fn enum_variant_unit() {
        let c = EnumSimple::Unit;

        let value = to_value(&c).unwrap();

        assert_eq!(value, Value::EnumVariantUnit(String::from("Unit")));
    }

    #[test]
    fn enum_variant_tuple() {
        let c = EnumSimple::Tuple("hello".into(), 3);

        let value = to_value(&c).unwrap();

        assert_eq!(
            value,
            Value::EnumVariantTuple(
                String::from("Tuple"),
                vec![Value::String("hello".into()), Value::Number(Number::I32(3))]
            )
        );
    }

    #[test]
    fn enum_variant_struct() {
        let c = EnumSimple::Struct {
            b: false,
            s: "hello".into(),
        };

        let value = to_value(&c).unwrap();

        assert_eq!(
            value,
            Value::EnumVariantStruct(
                String::from("Struct"),
                [
                    ("b", Value::Bool(false)),
                    ("s", Value::String("hello".into()))
                ]
                .map(|(k, v)| (k.to_owned(), v))
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn struct_nested() {
        let c = StructNested {
            v: vec![ComplexNested::new(
                "hello",
                Some(ComplexNested::new("world", None, None)),
                Some(EnumNested::Unit),
            )],
            t: (
                "hello".into(),
                ComplexNested::new(
                    "c",
                    None,
                    Some(EnumNested::Tuple("c2".into(), (Complex::new("neested"), 0))),
                ),
            ),
            m: {
                let mut h = HashMap::new();
                h.insert(
                    "k".into(),
                    ComplexNested::new(
                        "c",
                        None,
                        Some(EnumNested::Struct {
                            c: Complex::new("hello"),
                            s: "world".into(),
                        }),
                    ),
                );
                h
            },
        };

        let value = to_value(&c).unwrap();

        dbg!(&value);
    }
}
