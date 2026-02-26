use std::collections::BTreeMap;

use facet::{Facet, ScalarType};
use facet_format::{FormatSerializer, ScalarValue, SerializeError};
use facet_reflect::Peek;

use crate::{Number, Value};

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
}

enum StackFrame {
    Struct {
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
        }
    }

    fn finish(self) -> Value {
        self.result.unwrap()
    }

    fn emit(&mut self, value: Value) {
        match self.stack.last_mut() {
            Some(StackFrame::Struct {
                name,
                elems,
                pending_key,
            }) => {
                let key = pending_key
                    .take()
                    .expect("emit called on struct without pending key");

                elems.insert(key, value);
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
        // dbg!(shape);

        self.stack.push(StackFrame::Struct {
            name: shape.type_identifier.to_owned(),
            elems: BTreeMap::new(),
            pending_key: None,
        });

        Ok(())
    }

    fn begin_struct(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn field_key(&mut self, key: &str) -> Result<(), Self::Error> {
        match self.stack.last_mut() {
            Some(StackFrame::Struct { pending_key, .. }) => {
                *pending_key = Some(key.to_owned());
                Ok(())
            }
            None => Err(ToValueError::new("field_key called outside of object")),
        }
    }

    fn end_struct(&mut self) -> Result<(), Self::Error> {
        match self.stack.pop() {
            Some(StackFrame::Struct { name, elems, .. }) => {
                self.emit(Value::Struct(name, elems));
                Ok(())
            }
            _ => Err(ToValueError::new(
                "end_struct called without matching begin_struct",
            )),
        }
    }

    fn begin_seq(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn end_seq(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn typed_scalar(
        &mut self,
        scalar_type: facet::ScalarType,
        value: Peek<'_, '_>,
    ) -> Result<(), Self::Error> {
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
            ScalarType::F32 => Value::Number(Number::F32(*value.get::<f32>().unwrap())),
            ScalarType::F64 => Value::Number(Number::F64(*value.get::<f64>().unwrap())),
            _ => {
                todo!("{:?}", scalar_type)
            }
        };

        self.emit(value);
        Ok(())
    }

    fn scalar(&mut self, scalar: ScalarValue<'_>) -> Result<(), Self::Error> {
        let value = match scalar {
            ScalarValue::Unit => Value::Unit,
            ScalarValue::Null => Value::Null,
            ScalarValue::Bool(v) => Value::Bool(v),
            ScalarValue::Char(v) => Value::Char(v),
            ScalarValue::I64(v) => Value::Number(Number::I64(v)),
            ScalarValue::U64(v) => Value::Number(Number::U64(v)),
            ScalarValue::I128(v) => Value::Number(Number::I128(v)),
            ScalarValue::U128(v) => Value::Number(Number::U128(v)),
            ScalarValue::F64(v) => Value::Number(Number::F64(v)),
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
    use std::collections::HashMap;

    use facet::Facet;

    use crate::{Value, serialize::to_value};

    #[derive(Facet, Debug)]
    struct SimpleStruct {
        u: (),
        opt: Option<u8>,
        b: bool,
        y: f64,
        z: i32,
        c: char,
        x: String,
        v: Vec<u8>,
        t: (String, String),
        m: HashMap<String, i32>,
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
        Struct { c: bool, s: String },
    }

    #[derive(Facet, Debug)]
    struct Complex {
        x: String,
    }

    impl Complex {
        fn new(c: &str) -> Self {
            Self { x: c.into() }
        }
    }

    #[derive(Facet, Debug)]
    struct ComplexNested {
        x: Complex,
        b: Option<Box<ComplexNested>>,
        e: Option<Box<EnumNested>>,
    }

    impl ComplexNested {
        fn new(c: &str, o: Option<ComplexNested>, o2: Option<EnumNested>) -> Self {
            Self {
                x: Complex::new(c),
                b: o.map(Box::new),
                e: o2.map(Box::new),
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
            x: String::from("hello"),
            y: 3.66,
            z: 4,
            opt: None,
            c: '\n',
            b: false,
            u: (),
            v: vec![1, 2, 3],
            t: (String::from("hello"), String::from("world")),
            m: {
                let mut h = HashMap::new();
                h.insert("key1".into(), 6);
                h.insert("key2".into(), 7);
                h
            },
        };

        let value = to_value(&c).unwrap();

        dbg!(&value);
    }

    #[test]
    fn unit_struct() {
        let c = UnitStruct;

        let value = to_value(&c).unwrap();

        dbg!(&value);
    }

    #[test]
    fn tuple_struct() {
        let c = TupleStruct("hello".into(), 3);

        let value = to_value(&c).unwrap();

        dbg!(&value);
    }

    #[test]
    fn enum_variant_unit() {
        let c = EnumSimple::Unit;

        let value = to_value(&c).unwrap();

        dbg!(&value);
    }

    fn enum_variant_tuple() {
        let c = EnumSimple::Tuple("hello".into(), 3);

        let value = to_value(&c).unwrap();

        dbg!(&value);
    }

    fn enum_variant_struct() {
        let c = EnumSimple::Struct {
            c: false,
            s: "hello".into(),
        };

        let value = to_value(&c).unwrap();

        dbg!(&value);
    }

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
