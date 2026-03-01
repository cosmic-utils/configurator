use std::collections::BTreeMap;

use facet::Facet;

use crate::number::Number;

#[derive(Facet, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Value {
    Unit,
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
