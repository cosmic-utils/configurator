use std::collections::BTreeMap;

use anyhow::{anyhow, bail};

use indexmap::map::MutableKeys;

use crate::{
    generic_value::Value,
    node::{NodeArrayTemplate, NumberValue},
    utils::json_value_eq_value,
};

use super::{Node, NodeContainer};

impl NodeContainer {
    #[instrument(skip_all)]
    pub fn apply_value(&mut self, value: &Value, modified: bool) -> anyhow::Result<()> {
        debug!("\n{value:#?}\n{self:#?}\n{modified}");

        // debug!("merge_figment_rec {:?} {:?}", &self, &value);
        self.modified = modified;

        match &mut self.node {
            Node::Null => {
                // nothing to do ?
            }
            Node::Bool(node_bool) => {
                if let Some(value) = value.as_bool() {
                    node_bool.value = Some(*value)
                }
            }
            Node::String(node_string) => {
                if let Some(value) = value.as_str() {
                    node_string.value = Some(value.to_owned());
                }
            }
            Node::Number(node_number) => {
                if let Some(value) = value.as_number() {
                    let value = NumberValue::from_number(value);

                    node_number.value_string = value.to_string();
                    node_number.value = Some(value);
                }
            }
            Node::Value(node_value) => {
                // pass
            }
            Node::Object(node_object) => {
                // hashmap are overided by existence of a value
                // should this be in the remove_value_rec function ?
                node_object.nodes.retain(|_, node| !node.removable);

                if let Some((_, map)) = value.as_struct() {
                    // for known object field ?
                    for (key, n) in &mut node_object.nodes {
                        if let Some(value) = map.0.get(key) {
                            n.apply_value(value, modified)?;
                        } else if let Some(default) = &n.default {
                            n.apply_value(&default.clone(), false)?;
                        }
                    }

                    // for hashmap ?
                    if let Some(template) = node_object.template() {
                        for (key, value) in &map.0 {
                            let mut node_type = template.clone();
                            node_type.apply_value(value, modified)?;
                            node_object.nodes.insert(key.to_owned(), node_type);
                        }
                    }
                }

                if value.is_empty() {
                    for (key, n) in &mut node_object.nodes {
                        if let Some(default) = &n.default {
                            n.apply_value(&default.clone(), false)?;
                        }
                    }
                }
            }
            Node::Enum(node_enum) => {
                let pos = node_enum.nodes.iter().position(|e| e.is_matching(value));

                if let Some(pos) = pos {
                    node_enum.value = Some(pos);
                    node_enum.nodes[pos].apply_value(value, modified)?;
                } else {
                    panic!(
                        "can't find a compatible enum variant for \n{value:#?}.\n{node_enum:#?}"
                    );
                }
            }
            Node::Array(node_array) => {
                let vec = value
                    .as_list()
                    .or_else(|| value.as_tuple())
                    .or_else(|| value.as_named_tuple().map(|(_, v)| v));

                if let Some(vec) = vec {
                    let mut nodes = Vec::new();

                    for (pos, value) in vec.iter().enumerate() {
                        let mut new_node = node_array.template(Some(pos));
                        new_node.apply_value(value, modified)?;
                        nodes.push(new_node);
                    }

                    node_array.values = Some(nodes);
                }
            }
            Node::Any => todo!(),
        };

        Ok(())
    }

    #[instrument(skip_all)]
    fn is_matching(&self, value: &Value) -> bool {
        // debug!("\n{value:#?}\n{self:#?}\n");

        match &self.node {
            Node::Null => value.is_null(),
            Node::Bool(node_bool) => value.as_bool().is_some(),
            Node::String(node_string) => value.as_str().is_some(),
            Node::Number(node_number) => value.as_number().is_some(),
            Node::Object(node_object) => match value {
                Value::Struct(_, map) => node_object.nodes.iter().all(|(key, node)| {
                    map.0
                        .get(key)
                        .map(|value| node.is_matching(value))
                        .unwrap_or(false)
                }),
                Value::NamedTuple(name, _) => node_object
                    .nodes
                    .get(name)
                    .map(|node| node.is_matching(value))
                    .unwrap_or(false),
                _ => panic!("{self:#?}\n{value:#?}"),
            },
            Node::Enum(node_enum) => todo!(),
            Node::Array(node_array) => match &node_array.template {
                NodeArrayTemplate::All(node_container) => todo!(),
                NodeArrayTemplate::FirstN(node_containers) => {
                    let vec = value
                        .as_tuple()
                        .or_else(|| value.as_named_tuple().map(|(_, v)| v));

                    if let Some(vec) = vec {
                        node_containers.len() == vec.len()
                            && vec
                                .iter()
                                .zip(node_containers)
                                .all(|(value, node)| node.is_matching(value))
                    } else {
                        false
                    }
                }
            },
            Node::Value(node_value) => &node_value.value == value,
            Node::Any => todo!(),
        }
    }

    // todo: remove ALL values. Pass in each node container.
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
}
