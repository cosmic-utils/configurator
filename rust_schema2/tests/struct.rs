use std::default;

use rust_schema2::{RustSchema, schema_for};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod common;

#[test]
fn unit_struct() {
    /// Doc on Struct
    #[derive(RustSchema, Serialize, Deserialize, Debug)]
    struct A;

    test!(A);
}

#[test]
fn normal_struct() {
    /// Doc on Struct
    #[derive(RustSchema, Serialize, Deserialize, Debug)]
    #[serde(default)]
    struct A {
        /// Doc on field
        x: i32,
        #[serde(default)]
        y: String,
    }

    impl Default for A {
        fn default() -> Self {
            Self {
                x: 1,
                y: String::from("hello"),
            }
        }
    }

    test!(A);
}

#[test]
fn tuple_struct() {
    /// Doc on Struct
    #[derive(RustSchema, Serialize, Deserialize, Debug)]
    #[serde(default)]
    struct A(i32, String);

    impl Default for A {
        fn default() -> Self {
            Self(1, String::from("hello"))
        }
    }

    test!(A);
}

#[test]
fn new_type_struct() {
    /// Doc on Struct
    #[derive(RustSchema, Serialize, Deserialize, Debug)]
    #[serde(default)]
    struct A(String);

    impl Default for A {
        fn default() -> Self {
            Self(String::from("hello"))
        }
    }

    test!(A);
}
