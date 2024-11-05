use std::{borrow::Cow, collections::BTreeMap, fmt::Display};

use derive_more::derive::Unwrap;
use figment::value::{Num, Tag, Value};
use from_json_schema::json_value_to_figment_value;
use indexmap::IndexMap;
use light_enum::LightEnum;
use schemars::schema::SchemaObject;

use crate::utils::{figment_value_to_f64, figment_value_to_i128};

mod apply_figment;
pub mod data_path;
pub mod from_json_schema;
mod number;
pub use number::{NumberValue, NumberValueLight};
mod ser;
#[cfg(test)]
mod tests;
mod to_figment_value;

#[derive(Debug, Clone)]
pub struct NodeContainer {
    pub node: Node,
    pub default: Option<Value>,
    pub title: Option<String>,
    pub desc: Option<String>,
    /// Node that are modified should be written to disk
    pub modified: bool,
    /// Used for HashMap. We need to know if the node
    /// was created by a "template"
    pub removable: bool,
}

impl NodeContainer {
    pub fn from_node(node: Node) -> Self {
        Self {
            node,
            default: None,
            title: None,
            desc: None,
            modified: false,
            removable: false,
        }
    }
}

#[derive(Debug, Clone, Unwrap)]
#[unwrap(ref_mut)]
pub enum Node {
    Null,
    Bool(NodeBool),
    String(NodeString),
    Number(NodeNumber),
    // Option(NodeOption),
    Object(NodeObject),
    Enum(NodeEnum),
    Array(NodeArray),
    /// represent a final value
    /// currently only string is supported
    Value(NodeValue),
    Any,
}

#[derive(Debug, Clone)]
pub struct UnNamedObject {
    pub values: Vec<NodeContainer>,
}

impl UnNamedObject {
    pub fn new(values: Vec<NodeContainer>) -> Self {
        Self { values }
    }
}

#[derive(Debug, Clone)]
pub struct NodeBool {
    pub value: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NodeString {
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NodeNumber {
    pub kind: NumberValueLight,
    pub value: Option<NumberValue>,
    pub value_string: String,
}

#[derive(Debug, Clone)]
pub struct NodeValue {
    pub value: json::Value,
}

#[derive(Debug, Clone)]
pub struct NodeEnum {
    pub value: Option<usize>,
    pub nodes: Vec<NodeContainer>,
}

#[derive(Debug, Clone, Default)]
pub struct NodeObject {
    pub nodes: IndexMap<String, NodeContainer>,
    pub template: Option<Box<NodeContainer>>,
}

#[derive(Debug, Clone)]
pub enum NodeArrayTemplate {
    All(Box<NodeContainer>),
    FirstN(Vec<NodeContainer>),
}

#[derive(Debug, Clone)]
pub struct NodeArray {
    pub values: Option<Vec<NodeContainer>>,
    pub template: NodeArrayTemplate,
    pub min: Option<u32>,
    pub max: Option<u32>,
}

impl NodeBool {
    pub fn new() -> Self {
        Self { value: None }
    }
}

impl NodeString {
    pub fn new() -> Self {
        Self { value: None }
    }
}

impl NodeValue {
    pub fn new(value: json::Value) -> Self {
        Self { value }
    }
}

impl NodeEnum {
    pub fn new(nodes: Vec<NodeContainer>) -> Self {
        Self { value: None, nodes }
    }
}

impl NodeObject {
    pub fn new(nodes: IndexMap<String, NodeContainer>, node_type: Option<NodeContainer>) -> Self {
        Self {
            nodes,
            template: node_type.map(Box::new),
        }
    }

    pub fn template(&self) -> Option<NodeContainer> {
        match &self.template {
            Some(template) => {
                let mut template = *template.clone();
                template.removable = true;
                Some(template)
            }
            None => None,
        }
    }
}

impl NodeArray {
    pub fn new_any() -> Self {
        Self {
            values: None,
            template: NodeArrayTemplate::All(Box::new(NodeContainer::from_node(Node::Any))),
            min: None,
            max: None,
        }
    }

    pub fn template(&self, n: Option<usize>) -> NodeContainer {
        match &self.template {
            NodeArrayTemplate::All(new_node) => {
                let mut new_node = (**new_node).clone();
                new_node.removable = true;
                new_node
            }
            NodeArrayTemplate::FirstN(vec) => {
                let n = match n {
                    Some(n) => n,
                    None => match &self.values {
                        Some(v) => v.len(),
                        None => 0,
                    },
                };

                vec[n].clone()
            }
        }
    }
}

impl NodeContainer {
    /// Return true if all active note have a value
    pub fn is_valid(&self) -> bool {
        match &self.node {
            Node::Null => true,
            Node::Bool(node_bool) => node_bool.value.is_some(),
            Node::String(node_string) => node_string.value.is_some(),
            Node::Number(node_number) => node_number.value.is_some(),
            Node::Object(node_object) => node_object.nodes.values().all(|n| n.is_valid()),
            Node::Enum(node_enum) => node_enum
                .value
                .is_some_and(|pos| node_enum.nodes[pos].is_valid()),
            Node::Array(node_array) => node_array
                .values
                .as_ref()
                .is_some_and(|values| values.iter().all(|n| n.is_valid())),
            Node::Value(node_value) => true,
            Node::Any => true,
        }
    }

    pub fn metadata(self, metadata: &Option<Box<schemars::schema::Metadata>>) -> Self {
        Self {
            default: metadata
                .as_ref()
                .and_then(|m| m.default.as_ref())
                .map(json_value_to_figment_value),
            title: metadata.as_ref().and_then(|m| m.title.clone()),
            desc: metadata.as_ref().and_then(|m| m.description.clone()),
            ..self
        }
    }

    pub fn name(&self) -> Option<Cow<'_, str>> {
        match &self.node {
            Node::Null => Some(Cow::Borrowed("Null")),
            Node::Bool(node_bool) => None,
            Node::String(node_string) => None,
            Node::Number(node_number) => None,
            Node::Object(node_object) => None,
            Node::Enum(node_enum) => None,
            Node::Array(node_array) => None,
            Node::Value(node_value) => node_value.value.as_str().map(Cow::Borrowed),
            Node::Any => Some(Cow::Borrowed("Any")),
        }
    }
}

impl NodeEnum {
    pub fn unwrap_value(&self) -> (usize, &NodeContainer) {
        let pos = match self.value {
            Some(pos) => pos,
            None => panic!(),
        };

        (pos, &self.nodes[pos])
    }
}
