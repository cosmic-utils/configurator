mod map;
mod number;

mod merge;

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
    List(Vec<Value>),
    Map(Map<Value, Value>),
    Tuple(Vec<Value>),
    UnitStruct(String),
    Struct(Option<String>, Map<String, Value>),
    NamedTuple(String, Vec<Value>),
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

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        if let Value::List(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
