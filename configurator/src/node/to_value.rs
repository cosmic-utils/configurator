use crate::{
    generic_value::{Map, Value},
    node::Node,
};

use super::{NodeContainer, NumberValue};

impl NodeContainer {
    // todo: return a result with info about the node missing a value
    pub fn to_value(&self) -> Option<Value> {
        if !self.modified {
            return None;
        }

        match &self.node {
            Node::Null => Some(Value::Option(None)),
            Node::Bool(node_bool) => node_bool.value.map(Value::Bool),
            Node::String(node_string) => node_string
                .value
                .as_ref()
                .map(|value| Value::String(value.clone())),
            Node::Number(node_number) => node_number
                .value
                .as_ref()
                .map(|value| Value::Number(value.to_number())),
            Node::Object(node_object) => {
                if node_object.template.is_some() {
                    let mut map = Map::new();

                    for (key, node) in &node_object.nodes {
                        if let Some(value) = node.to_value() {
                            map.0.insert(Value::String(key.to_owned()), value);
                        }
                    }
                    Some(Value::Map(map))
                } else {
                    let mut map = Map::new();

                    for (key, node) in &node_object.nodes {
                        if let Some(value) = node.to_value() {
                            map.0.insert(key.to_owned(), value);
                        }
                    }
                    Some(Value::Struct(None, map))
                }
            }
            Node::Enum(node_enum) => node_enum
                .value
                .and_then(|pos| node_enum.nodes[pos].to_value()),
            Node::Array(node_array) => node_array
                .values
                .as_ref()
                .map(|values| values.iter().map(|n| n.to_value().unwrap()).collect())
                .map(|values| {
                    if node_array.is_tuple() {
                        Value::Tuple(values)
                    } else {
                        Value::List(values)
                    }
                }),
            Node::Value(node_value) => Some(node_value.value.clone()),
            Node::Any => todo!(),
        }
    }
}
