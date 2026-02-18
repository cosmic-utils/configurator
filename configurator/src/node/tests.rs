use crate::{generic_value::Value, node::NodeContainer, setup_log_for_test, test_common::*};

use std::collections::HashMap;

use cosmic::iced_futures::backend::default;

use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

/// 1. Generate a node from schema
/// 2. Apply the default impl to it
/// 3. assert that the serialization equal the default val if is_default_complete is true
fn test_schema<S: JsonSchema + Default + Serialize>(is_default_complete: bool) {
    setup_log_for_test();

    let schema = schema_for!(S);

    // dbg!(&schema);

    let mut tree = NodeContainer::from_json_schema(&schema);

    let config1 = S::default();

    let ron = ron::to_string(&config1).unwrap();
    let ron_value = ron_value::from_str(&ron).unwrap();
    let value = crate::providers::cosmic_ron::ron_value_to_value(ron_value);

    // tree.apply_value(&value, true).unwrap();
    tree.apply_value(&Value::Empty, true).unwrap();

    let value_from_node = tree.to_value();

    let value_from_node = if is_default_complete {
        value_from_node.expect("no value found but is_default_complete is true")
    } else {
        assert!(value_from_node.is_none());

        return;
    };

    assert_eq!(value_from_node, value);
}

#[test]
fn test_bool() {
    test_schema::<TestBool>(true);
}

#[test]
fn test_string() {
    test_schema::<TestString>(true);
}

#[test]
fn test_number() {
    test_schema::<TestNumber>(true);
}

#[test]
fn test_float() {
    test_schema::<TestFloat>(true);
}

#[test]
fn test_enum_simple() {
    test_schema::<TestEnumSimple>(true);
}

#[test]
fn test_enum_complex() {
    test_schema::<TestEnumComplex>(true);
}

#[test]
fn test_option() {
    test_schema::<TestOption>(true);
}

#[test]
fn test_option_complex() {
    test_schema::<TestOptionComplex>(true);
}

#[test]
fn test_tuple() {
    test_schema::<TestTuple>(true);
}

#[test]
fn test_vec() {
    test_schema::<TestVec>(true);
}

#[test]
fn test_hash_map() {
    test_schema::<TestHashMap>(true);
}

#[test]
fn test_very_complex() {
    test_schema::<TestVeryComplex>(true);
}

#[test]
fn test_rec() {
    test_schema::<Rec>(true);
}
