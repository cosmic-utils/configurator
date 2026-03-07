use rust_schema2::{RustSchema, schema_for};
use schemars::JsonSchema;
use serde::Serialize;

#[test]
fn testing() {
    #[derive(RustSchema, JsonSchema, Serialize)]
    #[serde(default)]
    struct A {
        x: i32,
        y: Option<Box<B>>,
    }

    impl Default for A {
        fn default() -> Self {
            Self {
                x: 3,
                y: Some(Box::new(B::default())),
            }
        }
    }

    #[derive(RustSchema, JsonSchema, Serialize)]
    #[serde(default)]
    struct B {
        x: i32,
    }

    impl Default for B {
        fn default() -> Self {
            Self {
                x: Default::default(),
            }
        }
    }

    let schema = schema_for::<A>();

    let json = json::to_string_pretty(&schema).unwrap();

    println!("{json}");

    let schema = schemars::schema_for!(A);

    let json = json::to_string_pretty(&schema).unwrap();

    println!("");
    println!("{json}");
}
