use std::collections::BTreeMap;
use std::fmt::Display;

use serde::Serialize;

use crate::number::{F32, F64, Number};
use crate::value::Value;

#[derive(Debug)]
pub enum Error {}

pub fn to_value<T: Serialize>(value: T) -> Value {
    value.serialize(Serializer).unwrap()
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // error is uninhabited; this should never be called
        match *self {}
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(_msg: T) -> Self {
        // cannot create an instance
        unreachable!("serde serialization error: {}", _msg)
    }
}

impl serde::ser::StdError for Error {}

struct Serializer;

struct SerializeVec {
    elems: Vec<Value>,
}

struct SerializeTuple {
    elems: Vec<Value>,
}

struct SerializeTupleStruct {
    name: &'static str,
    elems: Vec<Value>,
}

struct SerializeTupleVariant {
    name: &'static str,
    elems: Vec<Value>,
}

struct SerializeMap {
    map: BTreeMap<String, Value>,
    next_key: Option<String>,
}

pub struct SerializeStruct {
    name: &'static str,
    map: BTreeMap<String, Value>,
}

struct SerializeStructVariant {
    name: &'static str,
    map: BTreeMap<String, Value>,
}

impl serde::Serializer for Serializer {
    type Ok = Value;

    type Error = Error;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::I8(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::I16(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::I32(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::I64(v)))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::I128(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::U8(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::U16(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::U32(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::U64(v)))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::U128(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::F32(F32(v))))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(Number::F64(F64(v))))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let elems = v.iter().map(|b| Value::Number((*b).into())).collect();
        Ok(Value::Array(elems))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitStruct(name.to_owned()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::EnumVariantUnit(variant.to_owned()))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let inner = value.serialize(self)?;
        Ok(Value::TupleStruct(name.to_owned(), vec![inner]))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let inner = value.serialize(self)?;
        Ok(Value::EnumVariantTuple(variant.to_owned(), vec![inner]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeVec {
            elems: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple {
            elems: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct {
            name,
            elems: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant {
            name: variant,
            elems: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: BTreeMap::new(),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct {
            name,
            map: BTreeMap::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant {
            name: variant,
            map: BTreeMap::new(),
        })
    }
}

// compound serializer implementations

impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Serializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.elems))
    }
}

impl serde::ser::SerializeTuple for SerializeTuple {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Serializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Tuple(self.elems))
    }
}

impl serde::ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Serializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleStruct(self.name.to_owned(), self.elems))
    }
}

impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Serializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::EnumVariantTuple(self.name.to_owned(), self.elems))
    }
}

impl serde::ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = key.serialize(Serializer)?;
        match v {
            Value::String(s) => {
                self.next_key = Some(s);
                Ok(())
            }
            _ => panic!("map key was not a string"),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Serializer)?;
        let key = self
            .next_key
            .take()
            .expect("serialize_value called before serialize_key");
        self.map.insert(key, v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.map))
    }
}

impl serde::ser::SerializeStruct for SerializeStruct {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Serializer)?;
        self.map.insert(key.to_owned(), v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct(self.name.to_owned(), self.map))
    }
}

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(Serializer)?;
        self.map.insert(key.to_owned(), v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::EnumVariantStruct(self.name.to_owned(), self.map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn simple_values() {
        #[derive(Serialize)]
        struct A {
            x: i32,
            y: Option<Box<B>>,
        }

        #[derive(Serialize)]
        struct B {
            x: i32,
        }

        let v = A {
            x: 1,
            y: Some(Box::new(B { x: 2 })),
        };

        let v = to_value(&v);

        dbg!(&v);
    }
}
