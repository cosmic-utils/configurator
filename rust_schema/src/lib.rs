use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use facet::Facet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

enum Value {
    Unit,
    Null,
    Bool(bool),
    Number(i32),
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

struct RustSchemaRoot {
    schema: RustSchemaOrRef,
    definitions: HashMap<RustSchemaId, RustSchema>,
}

struct RustSchema {
    kind: RustSchemaKind,
    default: Option<Value>,
}

type RustSchemaId = u64;

enum RustSchemaOrRef {
    Ref(RustSchemaId),
    Schema(RustSchema),
}

enum RustSchemaKind {
    Unit,
    Bool,
    Number,
    Char,
    String,
    Option(RustSchemaOrRef),
    Array(RustSchemaOrRef),
    Tuple(Vec<RustSchemaOrRef>),
    Map(RustSchemaOrRef),
    Struct(String, BTreeMap<String, RustSchemaOrRef>),
    TupleStruct(String, Vec<RustSchemaOrRef>),
    Enum(String, Vec<(String, EnumVariantKind)>),
}

enum EnumVariantKind {
    Unit,
    Tuple(Vec<RustSchemaOrRef>),
    Struct(BTreeMap<String, RustSchemaOrRef>),
}

fn gen_rust_schema() -> RustSchemaRoot {
    todo!()
}

enum A {
    A,
    B(u32),
    C { a: u32 },
}

#[derive(Facet, Debug, Serialize, Deserialize, JsonSchema, Default)]
struct Y {
    x: u32,
    c: Option<Box<Config>>,
}

#[derive(Facet, Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(default)]
struct Config {
    #[facet(default)]
    x: (u32, String),
    #[facet(default)]
    y: u32,
    c: Option<Box<Config>>,
    d: Y,
}

#[test]
fn it_works() {
    dbg!(Config::SHAPE);

    let config = Config::default();
}

#[test]
fn schema() {
    let schema = schemars::schema_for!(Config);

    let value = json::value::to_value(&schema).unwrap();

    let str = json::to_string_pretty(&value).unwrap();

    fs::write("schema2.json", str.as_bytes()).unwrap();
}
