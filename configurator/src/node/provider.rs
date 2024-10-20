use figment::{
    value::{Dict, Empty, Num, Tag, Value},
    Metadata, Profile, Provider,
};

use crate::node::Node;

use super::{from_json_schema::json_value_to_figment_value, NodeContainer, NumberValue};

impl Provider for NodeContainer {
    fn metadata(&self) -> figment::Metadata {
        Metadata::named("name")
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        let mut map = figment::value::Map::new();

        if let Some(value) = self.to_value(&Tag::Default) {
            map.insert(Profile::default(), value.into_dict().unwrap());
        }

        Ok(map)
    }
}

enum Error {}
impl NodeContainer {
    fn to_value(&self, tag: &Tag) -> Option<Value> {
        if !self.modified {
            return None;
        }

        match &self.node {
            Node::Null => Some(Value::Empty(*tag, Empty::None)),
            Node::Bool(node_bool) => node_bool.value.map(|value| Value::Bool(*tag, value)),
            Node::String(node_string) => node_string
                .value
                .as_ref()
                .map(|value| Value::String(*tag, value.clone())),
            Node::Number(node_number) => node_number.value.as_ref().map(|value| match value {
                NumberValue::I128(value) => Value::Num(*tag, Num::I128(*value)),
                NumberValue::F64(value) => Value::Num(*tag, Num::F64(*value)),
            }),
            Node::Object(node_object) => {
                let mut dict = Dict::new();

                for (key, node) in &node_object.nodes {
                    if let Some(value) = node.to_value(tag) {
                        dict.insert(key.clone(), value);
                    }
                }
                Some(Value::Dict(*tag, dict))
            }
            Node::Enum(node_enum) => node_enum.value.and_then(|pos| {
                node_enum.nodes[pos].to_value(tag)

                // Value::Dict(tag.clone(), Dict::new());
                // todo!()
            }),
            Node::Array(node_array) => match &node_array.values {
                Some(values) => Some(Value::Array(
                    *tag,
                    values.iter().map(|n| n.to_value(tag).unwrap()).collect(),
                )),
                None => None,
            },
            Node::Value(node_value) => Some(json_value_to_figment_value(&node_value.value)),
        }
    }
}

#[cfg(test)]
mod test {

    use figment::{providers, Figment, Profile, Provider};
    use schemars::{schema_for, JsonSchema};
    use serde::Serialize;

    use crate::node::NodeContainer;

    #[derive(Debug, Serialize, JsonSchema)]
    struct A {
        e: E,
        bool: bool,
    }

    #[derive(Debug, Serialize, JsonSchema)]
    enum E {
        F(B),
    }

    #[derive(Debug, Serialize, JsonSchema)]
    struct B {
        k: String,
    }

    impl Default for A {
        fn default() -> Self {
            Self {
                e: E::F(B { k: "kaka".into() }),
                bool: false,
            }
        }
    }

    #[test]
    fn test() {
        let schema = schema_for!(A);

        let mut node = NodeContainer::from_json_schema(&schema);

        dbg!(&node);

        let default =
            Figment::new().merge(providers::Serialized::from(A::default(), Profile::Default));

        node.apply_figment(&default).unwrap();

        let from_node = Figment::new().merge(&node);

        dbg!(&default.data().unwrap());

        dbg!(&from_node.data().unwrap());
    }
}
