use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use serde::{
    ser::{SerializeMap, SerializeStruct, SerializeTuple},
    Serialize,
};

use super::NodeContainer;

impl Serialize for NodeContainer {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.node {
            super::Node::Null => todo!(),
            super::Node::Bool(node_bool) => ser.serialize_bool(node_bool.value.unwrap()),
            super::Node::String(node_string) => todo!(),
            super::Node::Number(node_number) => todo!(),
            super::Node::Object(node_object) => {
                let mut map = ser.serialize_struct("", node_object.nodes.len())?;

                for (key, val) in &node_object.nodes {
                    let key: &'static str = Box::leak(key.to_string().into_boxed_str());

                    map.serialize_field(key, val)?;
                }

                map.end()
            }
            super::Node::Enum(node_enum) => todo!(),
            super::Node::Array(node_array) => todo!(),
            super::Node::Value(node_value) => todo!(),
            super::Node::Any => todo!(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::test_common::*;

    use super::NodeContainer;
    use figment::{providers, value::Tag, Figment, Profile};
    use schemars::{schema_for, JsonSchema};
    use serde::{Deserialize, Serialize};

    fn test_schema<S: JsonSchema + Default + Serialize>() {
        let schema = schema_for!(S);

        let mut tree = NodeContainer::from_json_schema(&schema);

        let config1 = S::default();

        let figment = Figment::new().join(providers::Serialized::from(&config1, Profile::Default));

        tree.apply_figment(&figment).unwrap();

        let str1 = ron::ser::to_string_pretty(&config1, ron::ser::PrettyConfig::new()).unwrap();

        let str2 = ron::ser::to_string_pretty(&tree, ron::ser::PrettyConfig::new()).unwrap();

        assert_eq!(str1, str2);
    }

    #[test]
    fn test_bool_ron() {
        test_schema::<TestBool>();
    }
}
