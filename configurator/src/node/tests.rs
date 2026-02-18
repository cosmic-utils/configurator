use crate::{generic_value::Value, node::NodeContainer, setup_log_for_test, test_common::*};

use std::collections::HashMap;

use cosmic::iced_futures::backend::default;

use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

/// 1. Generate a node from schema
/// 2. Apply the default impl to it
/// 3. assert that the serialization equal the default val if is_default_complete is true
fn test_schema<S: JsonSchema + Serialize>(config1: &S, is_default_complete: bool) {
    setup_log_for_test();

    let schema = schema_for!(S);

    dbg!(&schema);

    let mut tree = NodeContainer::from_json_schema(&schema);

    dbg!(&tree);

    let ron = ron::to_string(&config1).unwrap();

    let ron_value = ron_value::from_str(&ron).unwrap();
    let value = crate::providers::cosmic_ron::ron_value_to_value(ron_value);

    dbg!(&value);

    tree.apply_value(&value, true).unwrap();
    // tree.apply_value(&Value::Empty, true).unwrap();

    dbg!(&tree);

    let value_from_node = tree.to_value();

    let value_from_node = if is_default_complete {
        value_from_node.expect("no value found but is_default_complete is true")
    } else {
        assert!(value_from_node.is_none());
        return;
    };

    dbg!(&value, &value_from_node);

    let ron_from_node = ron_value::to_string(&crate::providers::cosmic_ron::value_to_ron_value(
        value_from_node.clone(),
    ))
    .unwrap();

    // let value

    dbg!(&ron, &ron_from_node);

    assert_eq!(value_from_node, value);
}

#[test]
fn test_bool() {
    test_schema(&TestBool::default(), true);
}

#[test]
fn test_string() {
    test_schema(&TestString::default(), true);
}

#[test]
fn test_number() {
    test_schema(&TestNumber::default(), true);
}

#[test]
fn test_float() {
    test_schema(&TestFloat::default(), true);
}

#[test]
fn test_enum_simple() {
    test_schema(&TestEnumSimple::default(), true);
}

#[test]
fn test_option() {
    test_schema(&TestOption::default(), true);
}

#[test]
fn test_tuple() {
    test_schema(&TestTuple::default(), true);
}

#[test]
fn test_vec() {
    test_schema(&TestVec::default(), true);
}

#[test]
fn test_hash_map() {
    test_schema(&TestHashMap::default(), true);
}

#[test]
fn test_option_complex() {
    test_schema(&TestOptionComplex::default(), true);
}

#[test]
fn test_complex() {
    test_schema(&TestComplex::default(), true);
}

// marche pas
#[test]
fn test_enum_complex_tuple0() {
    test_schema(&TestEnumComplex { x: EnumComplex::A }, true);
}

#[test]
fn test_enum_complex_tuple1() {
    test_schema(
        &TestEnumComplex {
            x: EnumComplex::B(1),
        },
        true,
    );
}

#[test]
fn test_enum_complex_tuple2() {
    test_schema(
        &TestEnumComplex {
            x: EnumComplex::C(Complex::default(), 1),
        },
        true,
    );
}

#[test]
fn test_enum_complex_tuple_struct_like() {
    test_schema(
        &TestEnumComplex {
            x: EnumComplex::D {
                a: 1,
                b: Complex::default(),
            },
        },
        true,
    );
}

// marche pas
#[test]
fn test_very_complex() {
    test_schema(&TestVeryComplex::default(), true);
}

// marche pas
// #[test]
// fn test_rec() {
//     test_schema::<Rec>(true);
// }
