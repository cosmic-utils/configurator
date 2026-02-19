use std::collections::BTreeMap;

use facet::Facet;
use serde::{Deserialize, Serialize};

enum Value {
    Unit,
    Bool(bool),
    Number(i32),
    Char(char),
    String(String),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    UnitStruct(String),
    Struct(String, BTreeMap<String, Value>),
    TupleStruct(String, Vec<Value>),
    EnumVariantUnit(String),
    EnumVariantTuple(String, Vec<Value>),
    EnumVariantStruct(String, BTreeMap<String, Value>),
}

type RustSchemaId = u64;

struct RustSchema {
    kind: RustSchemaKind,
}

enum RustSchemaKind {
    Unit,
    Bool,
    Number,
    Char,
    String,
    Array(RustSchemaId),
    Tuple(Vec<RustSchemaId>),
    Map(RustSchemaId, RustSchemaId),
    Struct(String, BTreeMap<String, RustSchemaId>),
    TupleStruct(String, Vec<RustSchemaId>),
    Enum(String, Vec<(String, EnumVariantKind)>),
}

enum EnumVariantKind {
    Unit,
    Tuple(Vec<RustSchemaId>),
    Struct(BTreeMap<String, RustSchemaId>),
}

enum A {
    A,
    B(u32),
    C { a: u32 },
}

#[derive(Facet, Debug, Serialize, Deserialize)]
#[serde(default)]
struct Config {
    #[facet(default)]
    x: (u32, String),
    #[facet(default)]
    y: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: 88,
        }
    }
}

#[test]
fn it_works() {
    dbg!(Config::SHAPE);

    let config = Config::default();
}
