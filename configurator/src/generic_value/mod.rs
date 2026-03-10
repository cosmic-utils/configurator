mod map;
mod number;

mod merge;

use std::ops::Deref;

pub use number::{F32, F64, Number};

pub use map::Map;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Value {
    /// Represent the absence of value.
    Empty,
    Unit,
    Bool(bool),
    Char(char),
    Number(Number),
    String(String),
    Bytes(Vec<u8>),
    Option(Option<Box<Value>>),
    Array(Vec<Value>),
    Map(Map<String, Value>),
    Tuple(Vec<Value>),
    UnitStruct(String),
    Struct(Option<String>, Map<String, Value>),
    TupleStruct(String, Vec<Value>),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_owned())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(Number::I32(value))
    }
}

impl Value {
    pub fn as_bool(&self) -> Option<&bool> {
        if let Value::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<&Number> {
        if let Value::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_struct(&self) -> Option<(&Option<String>, &Map<String, Value>)> {
        if let Value::Struct(name, v) = self {
            Some((name, v))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Value::Empty
    }
    pub fn is_not_empty(&self) -> bool {
        self != &Value::Empty
    }

    pub fn if_not_empty<'a>(&'a self, fallback: &'a Value) -> &'a Value {
        self.is_not_empty().then_some(self).unwrap_or(fallback)
    }

    pub fn is_unit(&self) -> bool {
        self == &Value::Unit
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_tuple(&self) -> Option<&Vec<Value>> {
        if let Value::Tuple(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_tuple_struct(&self) -> Option<(&String, &Vec<Value>)> {
        if let Value::TupleStruct(name, v) = self {
            Some((name, v))
        } else {
            None
        }
    }

    pub fn is_null(&self) -> bool {
        self == &Value::Option(None)
    }

    pub fn as_option(&self) -> Option<&Option<Box<Value>>> {
        if let Value::Option(opt) = self {
            Some(opt)
        } else {
            None
        }
    }

    pub fn as_unit_struct(&self) -> Option<&str> {
        if let Value::UnitStruct(name) = self {
            Some(name)
        } else {
            None
        }
    }
}
