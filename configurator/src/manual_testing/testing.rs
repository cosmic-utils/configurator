use rust_schema2::RustSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, RustSchema, Serialize, Deserialize)]
#[serde(default)]
/// Desc A
struct A {
    /// Desc v
    v: Vec<B>,
    /// Desc x
    x: String,
    /// Desc y
    y: String,
    /// Desc z
    z: String,
    d: String,
}

impl Default for A {
    fn default() -> Self {
        A {
            v: vec![B { x: "hello1".into() }],
            x: String::default(),
            y: String::default(),
            z: String::default(),
            d: String::from("from default"),
        }
    }
}

#[derive(Clone, Debug, RustSchema, Serialize, Deserialize)]
struct B {
    x: String,
}

#[test]
#[ignore]
fn gen_schema() {
    super::gen_schema::<A>("testing");
}

#[test]
#[ignore]
fn print_json() {
    super::print_json::<A>();
}

#[test]
#[ignore]
fn print_ron() {
    super::print_ron::<A>();
}

#[test]
#[ignore]
fn from_ron() {
    super::from_ron::<A>("");
}

#[test]
#[ignore]
fn from_json() {
    super::from_json::<A>("");
}

#[test]
#[ignore]
fn from_json2() {
    super::from_json::<A>("");
}
