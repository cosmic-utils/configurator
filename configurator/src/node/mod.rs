use std::{borrow::Cow, collections::BTreeMap, fmt::Display};

use derive_more::derive::Unwrap;
use figment::{
    value::{Tag, Value},
    Profile, Provider,
};
use from_json_schema::json_value_to_figment_value;
use schemars::schema::SchemaObject;

use crate::utils::{figment_value_to_f64, figment_value_to_i128};

#[cfg(test)]
mod test;

mod apply_figment;
pub mod data_path;
pub mod from_json_schema;
mod provider;

#[derive(Debug, Clone)]
pub struct NodeContainer {
    pub node: Node,
    pub default: Option<Value>,
    pub title: Option<String>,
    pub desc: Option<String>,
    pub modified: bool,
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
pub enum NumberValue {
    I128(i128),
    F64(f64),
}

impl Display for NumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberValue::I128(n) => write!(f, "{}", n),
            NumberValue::F64(n) => write!(f, "{:.3}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NumberKind {
    Integer,
    Float,
}

#[derive(Debug, Clone)]
pub struct NodeNumber {
    pub kind: NumberKind,
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

#[derive(Debug, Clone)]
pub struct NodeObject {
    pub nodes: BTreeMap<String, NodeContainer>,
    pub node_type: Option<Box<NodeContainer>>,
}

#[derive(Debug, Clone)]
pub struct NodeArray {
    pub values: Option<Vec<NodeContainer>>,
    pub node_type: Box<NodeContainer>,
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

impl NodeNumber {
    pub fn new(kind: NumberKind) -> Self {
        Self {
            value: None,
            value_string: String::new(),
            kind,
        }
    }

    pub fn parse_number(&self, value: figment::value::Num) -> Option<NumberValue> {
        match self.kind {
            NumberKind::Integer => {
                figment_value_to_i128(&Value::Num(Tag::Default, value)).map(NumberValue::I128)
            }
            NumberKind::Float => {
                figment_value_to_f64(&Value::Num(Tag::Default, value)).map(NumberValue::F64)
            }
        }
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
    pub fn new(nodes: BTreeMap<String, NodeContainer>, node_type: Option<NodeContainer>) -> Self {
        Self {
            nodes,
            node_type: node_type.map(Box::new),
        }
    }
}

impl NodeArray {
    pub fn new(node_type: NodeContainer) -> Self {
        Self {
            node_type: Box::new(node_type),
            values: None,
        }
    }
}

impl NodeContainer {
    // only used for test rn
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
        }
    }

    pub fn from_metadata(node: Node, metadata: &Option<Box<schemars::schema::Metadata>>) -> Self {
        Self {
            node,
            default: None,
            title: None,
            desc: None,
            modified: false,
        }
        .set_metadata(metadata)
    }

    pub fn set_metadata(self, metadata: &Option<Box<schemars::schema::Metadata>>) -> Self {
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

fn custom_fmt(obj: &SchemaObject) {
    if let Some(obj) = &obj.object {
        dbg!(obj);
    } else if let Some(obj) = &obj.instance_type {
        dbg!(obj);
    }
}
