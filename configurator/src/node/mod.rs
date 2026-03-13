use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Display,
    rc::Rc,
};

use derive_more::derive::Unwrap;
use indexmap::IndexMap;
use light_enum::LightEnum;
use rust_schema2::{NumberKind, RustSchema, RustSchemaKind, RustSchemaOrRef, RustSchemaRoot};

use crate::{
    generic_value::{F32, F64, Map, Number, Value},
    node::data_path::DataPathType,
};

use anyhow::{anyhow, bail};

pub mod data_path;
// #[cfg(test)]
// mod tests;

mod from_schema_and_value;
mod set_modified;
mod to_value;

#[derive(Debug)]
pub struct NodeContainer {
    pub name: Option<String>,
    pub description: Option<String>,
    pub modified: bool,
    pub is_removable: bool,
    pub default: Value,
    pub node: Node,
}

#[derive(Debug, Unwrap)]
#[unwrap(ref_mut)]
pub enum Node {
    String(NodeString),
    Array(NodeArray),
    Struct(NodeStruct),
}

#[derive(Debug)]
pub struct NodeString {
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct NodeStruct {
    pub fields: IndexMap<String, NodeContainer>,
}

#[derive(Debug)]
pub struct NodeArray {
    pub min: Option<u64>,
    pub max: Option<u64>,
    pub value: Option<Vec<NodeContainer>>,
    pub has_template: bool,
}

impl NodeContainer {
    pub fn from_node(node: Node) -> Self {
        Self {
            node,
            modified: false,
            name: None,
            description: None,
            is_removable: false,
            default: Value::Empty,
        }
    }

    pub fn set_name(self, name: Option<String>) -> Self {
        Self { name, ..self }
    }

    pub fn set_description(self, description: Option<String>) -> Self {
        Self {
            description,
            ..self
        }
    }

    pub fn set_is_removable(self, is_removable: bool) -> Self {
        Self {
            is_removable,
            ..self
        }
    }

    pub fn set_default(self, default: Value) -> Self {
        Self { default, ..self }
    }

    pub fn remove_value_rec(&mut self) {
        self.modified = false;
        match &mut self.node {
            Node::String(node_string) => {
                node_string.value.take();
            }
            Node::Struct(node_struct) => {
                node_struct
                    .fields
                    .values_mut()
                    .for_each(|field| field.remove_value_rec());
            }
            Node::Array(node_array) => {
                node_array.value.take();
            }
        };
    }

    /// Return true if all active nodes have a value
    pub fn is_valid(&self) -> bool {
        match &self.node {
            Node::String(node_string) => node_string.value.is_some(),
            Node::Struct(node_struct) => node_struct.fields.values().all(|f| f.is_valid()),
            Node::Array(node_array) => node_array.value.as_ref().is_some_and(|values| {
                let is_complete = node_array
                    .min
                    .map(|min| values.len() >= min as usize)
                    .unwrap_or(true)
                    && node_array
                        .max
                        .map(|max| values.len() <= max as usize)
                        .unwrap_or(true);

                is_complete && values.iter().all(|n| n.is_valid())
            }),
        }
    }
}

pub fn schema_at<'a>(
    root: &'a RustSchemaRoot,
    data_path: &[DataPathType],
) -> anyhow::Result<&'a RustSchema> {
    let mut schema = root.resolve_schema(&root.schema)?;

    for data in data_path {
        match (&schema.kind, data) {
            (RustSchemaKind::Option(rust_schema_or_ref), DataPathType::Name(_)) => todo!(),
            (RustSchemaKind::Option(rust_schema_or_ref), DataPathType::Indice(_)) => todo!(),
            (RustSchemaKind::Array(array), DataPathType::Indice(_)) => match &array.template {
                Some(kind) => {
                    schema = root.resolve_schema(kind)?;
                }
                None => bail!("no kind for array: {:?}", schema),
            },
            (RustSchemaKind::Tuple(rust_schema_or_refs), DataPathType::Indice(_)) => todo!(),
            (RustSchemaKind::Map(rust_schema_or_ref), DataPathType::Name(_)) => todo!(),
            (RustSchemaKind::Struct(struct_), DataPathType::Name(name)) => {
                match struct_.fields.get(name) {
                    Some(field) => {
                        schema = root.resolve_schema(&field.schema)?;
                    }
                    None => {
                        bail!("no field named {} in {}", name, struct_.name)
                    }
                }
            }
            (RustSchemaKind::TupleStruct(tuple_struct), DataPathType::Indice(_)) => todo!(),
            (RustSchemaKind::Enum(_), DataPathType::Name(_)) => todo!(),
            (RustSchemaKind::Enum(_), DataPathType::Indice(_)) => todo!(),
            _ => bail!("schema {:?} is not compatible with {}", schema, data),
        }
    }

    Ok(schema)
}

fn value_at<'a>(value: &'a Value, data_path: &[DataPathType]) -> &'a Value {
    let mut value = value;

    for data in data_path {
        value = match (value, data) {
            (Value::Option(value), DataPathType::Name(_)) => todo!(),
            (Value::Option(value), DataPathType::Indice(_)) => todo!(),
            (Value::Array(values), DataPathType::Name(_)) => todo!(),
            (Value::Array(values), DataPathType::Indice(_)) => todo!(),
            (Value::Map(map), DataPathType::Name(_)) => todo!(),
            (Value::Map(map), DataPathType::Indice(_)) => todo!(),
            (Value::Tuple(values), DataPathType::Name(_)) => todo!(),
            (Value::Tuple(values), DataPathType::Indice(_)) => todo!(),
            (Value::UnitStruct(_), DataPathType::Name(_)) => todo!(),
            (Value::UnitStruct(_), DataPathType::Indice(_)) => todo!(),
            (Value::Struct(_, map), DataPathType::Name(name))
                if let Some(value) = map.0.get(name) =>
            {
                value
            }
            (Value::TupleStruct(_, values), DataPathType::Indice(i))
                if let Some(value) = values.get(*i) =>
            {
                value
            }
            _ => return &Value::Empty,
        };
    }

    value
}

fn rust_schema_value_to_value(value: &rust_schema2::Value) -> Value {
    match value {
        rust_schema2::Value::Unit => Value::Unit,
        rust_schema2::Value::Null => Value::Option(None),
        rust_schema2::Value::Bool(bool) => Value::Bool(*bool),
        rust_schema2::Value::Number(number) => Value::Number(match number {
            rust_schema2::Number::U8(v) => Number::U8(*v),
            rust_schema2::Number::U16(v) => Number::U16(*v),
            rust_schema2::Number::U32(v) => Number::U32(*v),
            rust_schema2::Number::U64(v) => Number::U64(*v),
            rust_schema2::Number::U128(v) => Number::U128(*v),
            rust_schema2::Number::USize(v) => Number::USize(*v),
            rust_schema2::Number::I8(v) => Number::I8(*v),
            rust_schema2::Number::I16(v) => Number::I16(*v),
            rust_schema2::Number::I32(v) => Number::I32(*v),
            rust_schema2::Number::I64(v) => Number::I64(*v),
            rust_schema2::Number::I128(v) => Number::I128(*v),
            rust_schema2::Number::ISize(v) => Number::ISize(*v),
            rust_schema2::Number::F32(rust_schema2::F32(v)) => Number::F32(F32(*v)),
            rust_schema2::Number::F64(rust_schema2::F64(v)) => Number::F64(F64(*v)),
        }),
        rust_schema2::Value::Char(c) => Value::Char(*c),
        rust_schema2::Value::String(s) => Value::String(s.to_owned()),
        rust_schema2::Value::Array(values) => {
            Value::Array(values.iter().map(rust_schema_value_to_value).collect())
        }
        rust_schema2::Value::Tuple(values) => {
            Value::Tuple(values.iter().map(rust_schema_value_to_value).collect())
        }
        rust_schema2::Value::Map(btree_map) => Value::Map(
            btree_map
                .iter()
                .map(|(k, v)| (k.to_owned(), rust_schema_value_to_value(v)))
                .collect(),
        ),
        rust_schema2::Value::UnitStruct(name) => Value::UnitStruct(name.to_owned()),
        rust_schema2::Value::Struct(name, btree_map) => Value::Struct(
            Some(name.to_owned()),
            btree_map
                .iter()
                .map(|(k, v)| (k.to_owned(), rust_schema_value_to_value(v)))
                .collect(),
        ),
        rust_schema2::Value::TupleStruct(name, values) => Value::TupleStruct(
            name.to_owned(),
            values.iter().map(rust_schema_value_to_value).collect(),
        ),
        rust_schema2::Value::EnumVariantUnit(_) => todo!(),
        rust_schema2::Value::EnumVariantTuple(_, values) => todo!(),
        rust_schema2::Value::EnumVariantStruct(_, btree_map) => todo!(),
    }
}
