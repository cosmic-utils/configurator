use rust_schema2::{RustSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(RustSchema, Deserialize, Serialize)]
#[serde(default)]
struct A {
    b: B,
}

impl Default for A {
    fn default() -> Self {
        Self {
            b: B {
                s: String::from("hello1"),
            },
        }
    }
}

#[derive(RustSchema, Deserialize, Serialize)]
struct B {
    s: String,
}

impl Default for B {
    fn default() -> Self {
        Self {
            s: Default::default(),
        }
    }
}

#[test]
fn test() {
    let schema = schema_for::<A>();

    let res = schema.assert_default_no_conflict();

    if let Err(e) = &res {
        println!("{e}");
    }

    res.unwrap_err();
}
