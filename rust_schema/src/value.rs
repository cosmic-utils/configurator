use std::collections::BTreeMap;

use crate::number::Number;

#[derive(Clone, Debug)]
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