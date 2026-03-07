use rust_schema2::{RustSchema, schema_for};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod common;

#[test]
fn struct_with_normal_field() {
    #[derive(RustSchema, Serialize, Deserialize, Debug)]
    struct A {
        x: i32,
    }

    test!(A);
}

#[test]
fn testing() {
    /// Bonsoir
    /// Second line
    #[derive(RustSchema, JsonSchema, Serialize, Deserialize, Debug)]
    #[serde(default)]
    struct A {
        /// Bonjour
        #[serde(default)]
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

    #[derive(RustSchema, JsonSchema, Serialize, Deserialize, Debug)]
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

    let a: A = json::from_str("{\"y\":null}").unwrap();

    dbg!(&a);
}
