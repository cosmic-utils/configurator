use crate::node::NodeContainer;

use std::collections::HashMap;

use figment::{
    providers,
    value::{Tag, Value},
    Figment, Profile, Provider,
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
struct Config1 {
    a: bool,
}

impl Config1 {
    fn new() -> Self {
        Self { a: false }
    }
}

#[test]
fn test_string() {
    let schema = schema_for!(Config1);
    let mut tree = NodeContainer::from_json_schema(&schema);

    let config1 = Config1::new();

    let figment = Figment::new().join(providers::Serialized::from(
        config1.clone(),
        Profile::Default,
    ));

    tree.apply_figment(&figment).unwrap();

    let value_from_node = tree.to_value(&Tag::Default).unwrap();

    let initial_value = Value::serialize(config1).unwrap();

    assert_eq!(value_from_node, initial_value);
}
