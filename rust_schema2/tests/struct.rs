use std::default;

use rust_schema2::{RustSchema, schema_for};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod common;

// #[test]
// fn unit_struct() {
//     #[derive(RustSchema, Serialize, Deserialize, Debug)]
//     struct A;

//     test!(A);
// }

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
