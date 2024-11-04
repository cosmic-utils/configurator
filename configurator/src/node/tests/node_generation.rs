use crate::node::NodeContainer;

use std::collections::HashMap;

use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
struct TestBool {
    bool: bool,
}

#[test]
fn test_bool() {
    let schema = schema_for!(TestBool);
    let _ = NodeContainer::from_json_schema(&schema);
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
struct TestString {
    string: String,
}

#[test]
fn test_string() {
    let schema = schema_for!(TestString);
    let _ = NodeContainer::from_json_schema(&schema);
}
