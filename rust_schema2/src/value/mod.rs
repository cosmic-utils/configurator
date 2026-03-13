use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod number;
mod ser;

pub use number::*;
pub use ser::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Unit,
    // todo: use option ?
    Null,
    Bool(bool),
    Number(Number),
    Char(char),
    String(String),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Map(BTreeMap<String, Value>),
    UnitStruct(String),
    Struct(String, BTreeMap<String, Value>),
    TupleStruct(String, Vec<Value>),
    EnumVariantUnit(String),
    EnumVariantTuple(String, Vec<Value>),
    EnumVariantStruct(String, BTreeMap<String, Value>),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_owned())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl Value {
    pub fn as_struct(&self) -> Option<(&String, &BTreeMap<String, Value>)> {
        if let Value::Struct(name, fields) = self {
            Some((name, fields))
        } else {
            None
        }
    }
}
