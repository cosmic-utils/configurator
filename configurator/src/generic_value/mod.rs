mod map;
mod number;

use number::Number;

use map::Map;

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
    Map(Map<Value, Value>),
    Tuple(Vec<Value>),
    UnitStruct(String),
    Struct(Option<String>, Map<String, Value>),
    NamedTuple(String, Vec<Value>),
}
