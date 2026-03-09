use crate::{
    generic_value::{Map, Value},
    node::Node,
};

use super::NodeContainer;

impl NodeContainer {
    pub fn to_value(&self) -> Option<Value> {
        if !self.modified {
            return None;
        }

        match &self.node {
            Node::String(node_string) => node_string
                .value
                .as_ref()
                .map(|value| Value::String(value.clone())),
            Node::Struct(node_struct) => {
                let mut map = Map::new();

                for (key, field) in &node_struct.fields {
                    if let Some(value) = field.node.to_value() {
                        map.0.insert(key.to_owned(), value);
                    }
                }
                Some(Value::Struct(Some(node_struct.name.to_owned()), map))
            }
            Node::Array(node_array) => {
                let mut values = Vec::new();

                if let Some(value) = &node_array.value {
                    values.reserve_exact(value.len());

                    for value in value {
                        values.push(value.to_value()?);
                    }
                }

                Some(Value::Array(values))
            }
        }
    }
}
