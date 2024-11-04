use figment::{
    value::{Dict, Num, Value},
    Figment, Provider,
};
use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
};

pub struct FigmentSerdeBridge<'a>(&'a Figment);

impl<'a> FigmentSerdeBridge<'a> {
    pub fn new(figment: &'a Figment) -> Self {
        Self(figment)
    }
}

struct DictBridge<'a>(&'a Dict);

impl Serialize for DictBridge<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_map(Some(self.0.len()))?;

        for (field_name, field) in self.0 {
            match field {
                Value::String(tag, value) => s.serialize_entry(field_name, value)?,
                Value::Char(tag, value) => s.serialize_entry(field_name, value)?,
                Value::Bool(tag, value) => s.serialize_entry(field_name, value)?,
                Value::Num(tag, value) => s.serialize_entry(field_name, value)?,
                Value::Empty(tag, empty) => todo!(),
                Value::Dict(tag, value) => s.serialize_entry(field_name, &DictBridge(value))?,
                Value::Array(tag, value) => s.serialize_entry(field_name, value)?,
            }
        }

        s.end()
    }
}

struct VecBridge<'a>(&'a Vec<Value>);

impl Serialize for VecBridge<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_seq(Some(self.0.len()))?;

        for value in self.0 {
            match value {
                Value::String(tag, value) => s.serialize_element(value)?,
                Value::Char(tag, value) => s.serialize_element(value)?,
                Value::Bool(tag, value) => s.serialize_element(value)?,
                Value::Num(tag, value) => s.serialize_element(value)?,
                Value::Empty(tag, empty) => todo!(),
                Value::Dict(tag, value) => {
                    s.serialize_element(&DictBridge(value))?;
                }
                Value::Array(tag, value) => {
                    s.serialize_element(&VecBridge(value))?;
                }
            }
        }

        s.end()
    }
}

struct NumBridge(Num);

impl Serialize for NumBridge {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Num::U8(v) => s.serialize_u8(v),
            Num::U16(v) => s.serialize_u16(v),
            Num::U32(v) => s.serialize_u32(v),
            Num::U64(v) => s.serialize_u64(v),
            Num::U128(v) => s.serialize_u128(v),
            Num::USize(v) => todo!(),
            Num::I8(v) => s.serialize_i8(v),
            Num::I16(v) => s.serialize_i16(v),
            Num::I32(v) => s.serialize_i32(v),
            Num::I64(v) => s.serialize_i64(v),
            Num::I128(v) => s.serialize_i128(v),
            Num::ISize(v) => todo!(),
            Num::F32(v) => s.serialize_f32(v),
            Num::F64(v) => s.serialize_f64(v),
        }
    }
}

impl Serialize for FigmentSerdeBridge<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let data = self.0.data().unwrap();

        let data = data.get(self.0.profile()).unwrap();

        data.serialize(serializer)
    }
}
