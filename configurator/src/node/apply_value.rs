use std::collections::BTreeMap;

use anyhow::{anyhow, bail};

use indexmap::map::MutableKeys;

use crate::{generic_value::Value, node::NumberValue, utils::json_value_eq_value};

use super::{Node, NodeContainer};

impl NodeContainer {
    #[instrument(skip_all)]
    pub fn apply_value(&mut self, value: &Value, modified: bool) -> anyhow::Result<()> {
        debug!("{self:#?}, {value:?}, {modified}");

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
            // todo: refractor this
            // we can probably include both case in is_matching
            Node::Enum(node_enum) => {
                if let Some((_, map)) = value.as_struct() {
                    let pos = map.0
                    .iter()
                    .find_map(|(key, value)| {
                        let key = Value::String(key.clone());
                        node_enum.nodes.iter().position(|e| e.is_matching(&key))
                    })
                    .ok_or_else(|| {
                        anyhow!(
                            "can't find a compatible enum variant for dict \n{map:#?}.\n{node_enum:#?}"
                        )
                    })?;

                    node_enum.value = Some(pos);
                    node_enum.nodes[pos].apply_value(value, modified)?;
                } else {
                    let pos = node_enum
                    .nodes
                    .iter()
                    .position(|e| e.is_matching(value))
                    .ok_or_else(|| {
                        anyhow!(
                            "can't find a compatible enum variant for \n{value:#?}.\n{node_enum:#?}"
                        )
                    })?;

                    node_enum.value = Some(pos);
                    node_enum.nodes[pos].apply_value(value, modified)?;
                }
            }
            Node::Array(node_array) => {
                if let Some(list) = value.as_list() {
                    let mut nodes = Vec::new();

                    for (pos, value) in list.iter().enumerate() {
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
                // todo: use zip here (and probably use the tuple variant)
                node_object.nodes.iter().enumerate().all(|(pos, (key, n))| {
                    let v = values.get(pos).unwrap();
                    n.is_matching(v)
                })
            }
            (Value::List(values), Node::Array(node_array)) => {
                // todo: more complicated logic
                true
            }
            (value, Node::Value(node_value)) => json_value_eq_value(&node_value.value, value),
            _ => false,
        }
    }
}
