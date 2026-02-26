use std::fs;

use facet::Facet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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


#[test]
fn shape() {

    #[derive(Facet)]
    struct T();


    dbg!(T::SHAPE);
}