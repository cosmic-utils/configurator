use std::borrow::Cow;

mod de;
mod map;
mod number;
mod ser;

#[cfg(test)]
mod test;

pub use map::Map;
pub use number::{F32, F64, Number};

pub use de::{DeserializeError, from_str};

pub use ser::{SerializeError, to_string};

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Value {
    Unit,
    Bool(bool),
    Char(char),
    Number(Number),
    String(String),
    Bytes(Vec<u8>),
    Option(Option<Box<Value>>),
    List(Vec<Value>),
    Map(Map<Value>),
    Tuple(Vec<Value>),

    UnitStructOrEnum(Cow<'static, str>),
    UnitEnum(Cow<'static, str>),
    UnitStruct(Cow<'static, str>),

    StructOrEnum(Option<Cow<'static, str>>, Map<Cow<'static, str>>),
    Struct(Option<Cow<'static, str>>, Map<Cow<'static, str>>),
    Enum(Cow<'static, str>, Map<Cow<'static, str>>),

    EnumTuple(Cow<'static, str>, Vec<Value>),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl<K: Into<Value>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::Map(iter.into_iter().collect())
    }
}

impl<T: Into<Number>> From<T> for Value {
    fn from(value: T) -> Self {
        Self::Number(value.into())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        Self::Option(value.map(Into::into).map(Box::new))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        String::from(value).into()
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    fn from(value: Cow<'a, str>) -> Self {
        String::from(value).into()
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

/// Special case to allow `Value::from(b"byte string")`
impl<const N: usize> From<&'static [u8; N]> for Value {
    fn from(value: &'static [u8; N]) -> Self {
        Self::Bytes(Vec::from(*value))
    }
}

impl<T: Into<Value>> FromIterator<T> for Value {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::List(iter.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
    fn from(value: &'a [T]) -> Self {
        value.iter().map(Clone::clone).map(Into::into).collect()
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        value.into_iter().collect()
    }
}

impl From<()> for Value {
    fn from(_value: ()) -> Self {
        Value::Unit
    }
}
