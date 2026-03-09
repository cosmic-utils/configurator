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
    node::data_path::{DataPathType, data_path_alloc_one},
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
    pub node: Node,
    pub modified: bool,
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
    pub name: String,
    pub description: Option<String>,
    pub fields: IndexMap<String, StructField>,
}

#[derive(Debug)]
pub struct StructField {
    pub description: Option<String>,
    pub node: NodeContainer,
}

#[derive(Debug)]
pub struct NodeArray {
    pub min: Option<u64>,
    pub max: Option<u64>,
    pub value: Option<Vec<NodeContainer>>,
}

impl NodeContainer {
    pub fn remove_value_rec(&mut self) {
        self.modified = false;
        match &mut self.node {
            Node::String(node_string) => {
                node_string.value.take();
            }
            Node::Struct(node_struct) => {
                // remove hashmap object ?
                node_struct
                    .fields
                    .values_mut()
                    .for_each(|field| field.node.remove_value_rec());
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
            Node::Struct(node_struct) => node_struct.fields.values().all(|f| f.node.is_valid()),
            Node::Array(node_array) => node_array
                .value
                .as_ref()
                .is_some_and(|values| values.iter().all(|n| n.is_valid())),
        }
    }

    // pub fn get_default(&self) -> Option<&Value> {
    //     match &self.node {
    //         Node::String(node_string) => None,
    //         Node::Array(node_array) => None,
    //         Node::Struct(node_struct) => node_struct.,
    //     }
    // }
}

pub fn schema_at<'a>(
    root: &'a RustSchemaRoot,
    data_path: &[DataPathType],
) -> anyhow::Result<&'a RustSchema> {
    let mut schema = get_schema(root, &root.schema)?;

    for data in data_path {
        match (&schema.kind, data) {
            (RustSchemaKind::Option(rust_schema_or_ref), DataPathType::Name(_)) => todo!(),
            (RustSchemaKind::Option(rust_schema_or_ref), DataPathType::Indice(_)) => todo!(),
            (RustSchemaKind::Array(array), DataPathType::Indice(_)) => match &array.kind {
                Some(kind) => {
                    schema = get_schema(root, kind)?;
                }
                None => bail!("no kind for array: {:?}", schema),
            },
            (RustSchemaKind::Tuple(rust_schema_or_refs), DataPathType::Indice(_)) => todo!(),
            (RustSchemaKind::Map(rust_schema_or_ref), DataPathType::Name(_)) => todo!(),
            (RustSchemaKind::Struct(struct_), DataPathType::Name(name)) => {
                match struct_.fields.get(name) {
                    Some(field) => {
                        schema = get_schema(root, &field.schema)?;
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

fn get_schema<'a>(
    root: &'a RustSchemaRoot,
    schema: &'a RustSchemaOrRef,
) -> anyhow::Result<&'a RustSchema> {
    root.get_schema(schema)
        .ok_or(anyhow!("unknow ref {:?}", schema))
}

fn rust_schema_value_to_value(value: &rust_schema2::Value) -> Value {
    match value {
        rust_schema2::Value::Unit => Value::Unit,
        rust_schema2::Value::Null => Value::Option(None),
        rust_schema2::Value::Bool(bool) => Value::Bool(*bool),
        rust_schema2::Value::Number(number) => Value::Number(match number {
            rust_schema2::Number::U8(_) => todo!(),
            rust_schema2::Number::U16(_) => todo!(),
            rust_schema2::Number::U32(_) => todo!(),
            rust_schema2::Number::U64(_) => todo!(),
            rust_schema2::Number::U128(_) => todo!(),
            rust_schema2::Number::USize(_) => todo!(),
            rust_schema2::Number::I8(_) => todo!(),
            rust_schema2::Number::I16(_) => todo!(),
            rust_schema2::Number::I32(v) => Number::I32(*v),
            rust_schema2::Number::I64(_) => todo!(),
            rust_schema2::Number::I128(_) => todo!(),
            rust_schema2::Number::ISize(_) => todo!(),
            rust_schema2::Number::F32(f32) => todo!(),
            rust_schema2::Number::F64(f64) => todo!(),
        }),
        rust_schema2::Value::Char(_) => todo!(),
        rust_schema2::Value::String(s) => Value::String(s.to_owned()),
        rust_schema2::Value::Array(values) => todo!(),
        rust_schema2::Value::Tuple(values) => todo!(),
        rust_schema2::Value::Map(btree_map) => todo!(),
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
