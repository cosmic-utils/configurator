use crate::{node::NodeContainer, test_common::*};

use std::collections::HashMap;

use cosmic::iced_futures::backend::default;
use figment::{
    providers,
    value::{Tag, Value},
    Figment, Profile,
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

/// 1. Generate a node from schema
/// 2. Apply the default impl to it
/// 3. assert that the serialization equal the default val if is_default_complete is true
fn test_schema<S: JsonSchema + Default + Serialize>(is_default_complete: bool) {
    let schema = schema_for!(S);

    let mut tree = NodeContainer::from_json_schema(&schema);

    let config1 = S::default();

    let figment = Figment::new().join(providers::Serialized::from(&config1, Profile::Default));

    tree.apply_figment(&figment).unwrap();

    let value_from_node = tree.to_value(&Tag::Default);

    let value_from_node = if is_default_complete {
        value_from_node.expect("no value found but is_default_complete is true")
    } else {
        assert!(value_from_node.is_none());

        return;
    };

    let initial_value = Value::serialize(&config1).unwrap();

    assert_eq!(value_from_node, initial_value);
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
