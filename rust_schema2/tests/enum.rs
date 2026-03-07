use std::default;

use rust_schema2::{RustSchema, schema_for};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod common;

#[test]
fn enum1() {
    /// Doc on Enum
    #[derive(RustSchema, Serialize, Deserialize, Debug)]
    enum A {
        /// Doc on variant
        Unit,
        
    }

    test!(A);
}
