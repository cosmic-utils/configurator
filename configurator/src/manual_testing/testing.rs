use rust_schema2::RustSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, RustSchema, Serialize, Deserialize)]
#[serde(default)]
struct A {
    x: B,
}

impl Default for A {
    fn default() -> Self {
        Self { x: B { x: 0 } }
    }
}

#[derive(Clone, Debug, RustSchema, Serialize, Deserialize)]
#[serde(default)]
struct B {
    x: i32,
}

impl Default for B {
    fn default() -> Self {
        B { x: 1 }
    }
}


#[test]
#[ignore]
fn print_json() {
    super::print_json::<A>();
}

#[test]
#[ignore]
fn from_json() {
    super::from_json::<A>("{}");
}

#[test]
#[ignore]
fn from_json2() {
    super::from_json::<A>("{\"x\":{}}");
}