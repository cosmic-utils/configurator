use std::collections::BTreeMap;

use anyhow::{anyhow, bail};
use figment::{
    value::{Tag, Value},
    Figment,
};
use indexmap::map::MutableKeys;

use crate::utils::{data_default_profile_figment, json_value_eq_figment_value};

use super::{Node, NodeContainer};

impl NodeContainer {
    pub fn apply_figment(&mut self, figment: &Figment) -> anyhow::Result<()> {
        match data_default_profile_figment(figment) {
            Some(data) => self.apply_value(Value::Dict(Tag::Default, data), true),
            None => self.apply_value(Value::Dict(Tag::Default, BTreeMap::new()), false),
        }
    }

    // todo: the modified logic in the function seems wrong (i probably fixed it)
    // todo2: analyze the entire logic
    pub fn apply_value(&mut self, value: Value, modified: bool) -> anyhow::Result<()> {
        // info!("merge_figment_rec");
        // dbg!(&self, &value);
        self.modified = modified;

        match (value, &mut self.node) {
            (Value::String(tag, value), Node::String(node_string)) => {
                node_string.value = Some(value);
            }
            (Value::String(tag, value), Node::Enum(node_enum)) => {
                let value = Value::String(tag, value);

                let pos = node_enum
                    .nodes
                    .iter()
                    .position(|e| e.is_matching(&value))
                    .ok_or_else(|| {
                        anyhow!("can't find a compatible enum variant for string {value:#?}. {node_enum:#?}")
                    })?;

                node_enum.value = Some(pos);
                node_enum.nodes[pos].apply_value(value, modified)?;
            }
            (Value::String(tag, value), Node::Value(node_value)) => {
                // pass
            }
            (Value::Bool(tag, value), Node::Bool(node_bool)) => node_bool.value = Some(value),
            (Value::Num(tag, value), Node::Number(node_number)) => {
                // dbg!(&value);

                let value = node_number.parse_number(value).unwrap();

                node_number.value_string = value.to_string();
                node_number.value = Some(value);
            }
            (Value::Empty(tag, value), Node::Enum(node_enum)) => {
                let value = Value::Empty(tag, value);

                let pos = node_enum
                    .nodes
                    .iter()
                    .position(|e| e.is_matching(&value))
                    .ok_or_else(|| {
                        anyhow!("can't find a compatible enum variant for empty {value:#?}. {node_enum:#?}")
                    })?;

                node_enum.value = Some(pos);
            }
            (Value::Dict(tag, mut values), Node::Object(node_object)) => {
                // hashmap are overided by existence of a value
                node_object.nodes.retain(|_, node| !node.removable);

                // for known object field ?
                for (key, n) in &mut node_object.nodes {
                    if let Some(value) = values.remove(key) {
                        n.apply_value(value, modified)?;
                    } else if let Some(default) = &n.default {
                        n.apply_value(default.clone(), false)?;
                    }
                }

                // for hashmap ?
                if let Some(template) = node_object.template() {
                    for (key, value) in values {
                        let mut node_type = template.clone();
                        node_type.apply_value(value, modified)?;
                        node_object.nodes.insert(key, node_type);
                    }
                }
            }
            (Value::Dict(tag, values), Node::Enum(node_enum)) => {
                let pos = values
                    .iter()
                    .find_map(|(key, value)| {
                        let key = Value::String(tag, key.clone());
                        node_enum.nodes.iter().position(|e| e.is_matching(&key))
                    })
                    .ok_or_else(|| {
                        anyhow!(
                            "can't find a compatible enum variant for dict {values:#?}. {node_enum:#?}"
                        )
                    })?;

                node_enum.value = Some(pos);
                node_enum.nodes[pos].apply_value(Value::Dict(tag, values), modified)?;
            }
            (Value::Array(tag, values), Node::Array(node_array)) => {
                let mut nodes = Vec::new();

                for value in values {
                    let mut new_node = node_array.template();
                    new_node.apply_value(value, modified)?;
                    nodes.push(new_node);
                }

                node_array.values = Some(nodes);
            }
            (value, node) => bail!("no compatible node for array. value = {value:#?}. {node:#?}"),
        };

        Ok(())
    }

    pub fn remove_value_rec(&mut self) {
        match &mut self.node {
            Node::Null => {}
            Node::Bool(node_bool) => {
                node_bool.value.take();
            }
            Node::String(node_string) => {
                node_string.value.take();
            }
            Node::Number(node_number) => {
                node_number.value.take();
            }
            Node::Object(node_object) => {
                // remove hashmap object ?
                node_object
                    .nodes
                    .values_mut()
                    .for_each(|node| node.remove_value_rec());
            }
            Node::Enum(node_enum) => {
                node_enum.value.take();
            }
            Node::Array(node_array) => {
                // is it safe ?
                node_array.values.take();
            }
            Node::Value(node_value) => {}
        };
        self.modified = false;
    }

    fn is_matching(&self, value: &Value) -> bool {
        // todo: should this match so many things ?
        // maybe only what is possible to put in an enum key
        // is it correct tho, maybe we should do a full equivalence on String
        match (value, &self.node) {
            (Value::String(tag, _), Node::String(node_string)) => true,
            (Value::String(tag, value), Node::Object(node_object)) => {
                node_object.nodes.contains_key(value)
            }
            (Value::Bool(tag, _), Node::Bool(node_bool)) => true,
            (Value::Num(tag, num), Node::Number(node_number)) => true,
            (Value::Empty(tag, empty), Node::Null) => true,
            (Value::Dict(tag, values), Node::Object(node_object)) => {
                node_object.nodes.iter().all(|(key, n)| {
                    let v = values.get(key).unwrap();
                    n.is_matching(v)
                })
            }
            (value, Node::Value(node_value)) => {
                json_value_eq_figment_value(&node_value.value, value)
            }
            _ => false,
        }
    }
}
