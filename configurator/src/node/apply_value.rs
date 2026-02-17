use std::collections::BTreeMap;

use anyhow::{anyhow, bail};

use indexmap::map::MutableKeys;

use crate::{generic_value::Value, utils::json_value_eq_value};

use super::{Node, NodeContainer};

impl NodeContainer {
    // todo: use figment Value instead
    pub fn apply_value(&mut self, value: &Value) -> anyhow::Result<()> {
        self.apply_value2(value, value == Value::Empty)
    }

    // todo: the modified logic in the function seems wrong (i probably fixed it)
    // todo2: analyze the entire logic
    pub fn apply_value2(&mut self, value: &Value, modified: bool) -> anyhow::Result<()> {
        // debug!("merge_figment_rec {:?} {:?}", &self, &value);
        self.modified = modified;

        match (value, &mut self.node) {
            (Value::String(value), Node::String(node_string)) => {
                node_string.value = Some(value.clone());
            }
            (Value::Struct(name, values), Node::Enum(node_enum)) => {
                let pos = values
                    .iter()
                    .find_map(|(key, value)| {
                        let key = Value::String(key.clone());
                        node_enum.nodes.iter().position(|e| e.is_matching(&key))
                    })
                    .ok_or_else(|| {
                        anyhow!(
                            "can't find a compatible enum variant for dict \n{values:#?}.\n{node_enum:#?}"
                        )
                    })?;

                node_enum.value = Some(pos);
                node_enum.nodes[pos].apply_value2(&value, modified)?;
            }
            (value, Node::Enum(node_enum)) => {
                let pos = node_enum
                    .nodes
                    .iter()
                    .position(|e| e.is_matching(&value))
                    .ok_or_else(|| {
                        anyhow!(
                            "can't find a compatible enum variant for \n{value:#?}.\n{node_enum:#?}"
                        )
                    })?;

                node_enum.value = Some(pos);
                node_enum.nodes[pos].apply_value2(value, modified)?;
            }
            (Value::String(value), Node::Value(node_value)) => {
                // pass
            }
            (Value::Bool(value), Node::Bool(node_bool)) => node_bool.value = Some(value.clone()),
            (Value::Number(value), Node::Number(node_number)) => {
                // dbg!(&value);
                // dbg!(&node_number);

                let value = node_number.try_from_number(value)?;

                node_number.value_string = value.to_string();
                node_number.value = Some(value);
            }
            (Value::Struct(tag, mut values), Node::Object(node_object)) => {
                // hashmap are overided by existence of a value
                node_object.nodes.retain(|_, node| !node.removable);

                // for known object field ?
                for (key, n) in &mut node_object.nodes {
                    if let Some(value) = values.remove(key) {
                        n.apply_value2(value, modified)?;
                    } else if let Some(default) = &n.default {
                        n.apply_value2(&default, false)?;
                    }
                }

                // for hashmap ?
                if let Some(template) = node_object.template() {
                    for (key, value) in values {
                        let mut node_type = template.clone();
                        node_type.apply_value2(value, modified)?;
                        node_object.nodes.insert(key, node_type);
                    }
                }
            }
            (Value::List(values), Node::Array(node_array)) => {
                let mut nodes = Vec::new();

                for (pos, value) in values.into_iter().enumerate() {
                    let mut new_node = node_array.template(Some(pos));
                    new_node.apply_value2(value, modified)?;
                    nodes.push(new_node);
                }

                node_array.values = Some(nodes);
            }
            (Value::Option(None), Node::Null) => {}
            (value, node) => bail!("no compatible node for value = \n{value:#?}. \n{node:#?}"),
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
            Node::Any => todo!(),
        };
        self.modified = false;
    }

    fn is_matching(&self, value: &Value) -> bool {
        // todo: should this match so many things ?
        // maybe only what is possible to put in an enum key
        // is it correct tho, maybe we should do a full equivalence on String
        match (value, &self.node) {
            (Value::String(_), Node::String(node_string)) => true,
            (Value::String(value), Node::Object(node_object)) => {
                node_object.nodes.contains_key(value)
            }
            (Value::Bool(_), Node::Bool(_)) => true,
            (Value::Number(_), Node::Number(_)) => true,
            (Value::Option(None), Node::Null) => true,
            (Value::List(values), Node::Object(node_object)) => {
                node_object.nodes.iter().all(|(key, n)| {
                    let v = values.get(key).unwrap();
                    n.is_matching(v)
                })
            }
            (Value::List(values), Node::Array(node_array)) => {
                // todo: more complicated logic
                true
            }
            (value, Node::Value(node_value)) => {
                json_value_eq_value(&node_value.value, value)
            }
            _ => false,
        }
    }
}
