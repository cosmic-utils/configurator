mod map;
mod number;

pub use number::{F32, F64, Number};

pub use map::Map;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Value {
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

impl Value {
    pub fn merge(&self, other: &Self) -> Self {
        todo!()
    }

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

    pub fn as_struct(&self) -> Option<&Map<String, Value>> {
        if let Value::Struct(_, v) = self {
            Some(v)
        } else {
            None
        }
    }
}
