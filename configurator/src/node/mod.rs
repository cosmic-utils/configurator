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
    pub tampon: String,
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

pub fn from_schema_and_value(
    root: &RustSchemaRoot,
    schema: &RustSchema,
    value: &Value,
    modified: bool,
) -> NodeContainer {
    match &schema.kind {
        RustSchemaKind::Unit => todo!(),
        RustSchemaKind::Boolean => todo!(),
        RustSchemaKind::Number(number_kind) => todo!(),
        RustSchemaKind::Char => todo!(),
        RustSchemaKind::String => {
            if let Some(str) = value.as_str() {
                NodeContainer {
                    node: Node::String(NodeString {
                        value: Some(str.to_owned()),
                        tampon: str.to_string(),
                    }),
                    modified,
                }
            } else {
                NodeContainer {
                    node: Node::String(NodeString {
                        value: None,
                        tampon: String::default(),
                    }),
                    modified,
                }
            }
        }
        RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
        RustSchemaKind::Array(array) => {
            if let Some(vec) = value.as_array() {
                let res = if let Some(template) = &array.kind {
                    let template = get_schema(root, template).unwrap();

                    vec.iter()
                        .map(|v| from_schema_and_value(root, template, v, modified))
                        .collect()
                } else {
                    vec![]
                };

                NodeContainer {
                    // is_incomplete: !(array
                    //     .min
                    //     .map(|min| res.len() >= min as usize)
                    //     .unwrap_or(true)
                    //     && array
                    //         .max
                    //         .map(|max| res.len() <= max as usize)
                    //         .unwrap_or(true)),
                    node: Node::Array(NodeArray {
                        min: array.min.clone(),
                        max: array.max.clone(),
                        value: Some(res),
                    }),
                    modified,
                }
            } else {
                NodeContainer {
                    node: Node::Array(NodeArray {
                        min: array.min.clone(),
                        max: array.max.clone(),
                        value: None,
                    }),
                    modified,
                }
            }
        }
        RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
        RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
        RustSchemaKind::Struct(struct_) => {
            if value.is_empty() {
                if let Some(struct_default) = &struct_.default {
                    // struct_default can't be empty
                    return from_schema_and_value(
                        root,
                        schema,
                        &rust_schema_value_to_value(struct_default),
                        false,
                    );
                }
            }

            if let Some((name, map)) = value.as_struct() {
                let mut fields = IndexMap::new();

                for (field_name, field) in &struct_.fields {
                    let schema = get_schema(root, &field.schema).unwrap();
                    let description = field.description.to_owned();

                    if let Some(field_default) = &field.default {
                        if let Some(field_value) = map.0.get(field_name)
                            && !modified
                        {
                            fields.insert(
                                field_name.to_owned(),
                                StructField {
                                    description,
                                    node: from_schema_and_value(
                                        root,
                                        schema,
                                        field_value,
                                        modified,
                                    ),
                                },
                            );
                        } else {
                            fields.insert(
                                field_name.to_owned(),
                                StructField {
                                    description,
                                    node: from_schema_and_value(
                                        root,
                                        schema,
                                        &rust_schema_value_to_value(field_default),
                                        false,
                                    ),
                                },
                            );
                        }
                    } else {
                        if let Some(field_value) = map.0.get(field_name) {
                            fields.insert(
                                field_name.to_owned(),
                                StructField {
                                    description,
                                    node: from_schema_and_value(
                                        root,
                                        schema,
                                        field_value,
                                        modified,
                                    ),
                                },
                            );
                        } else {
                            fields.insert(
                                field_name.to_owned(),
                                StructField {
                                    description,
                                    node: from_schema_and_value(root, schema, &Value::Empty, false),
                                },
                            );
                        }
                    }
                }

                return NodeContainer {
                    node: Node::Struct(NodeStruct {
                        name: struct_.name.to_owned(),
                        description: struct_.description.to_owned(),
                        fields,
                    }),
                    modified,
                };
            }

            for (field_name, field) in &struct_.fields {}

            NodeContainer {
                node: Node::Struct(NodeStruct {
                    name: struct_.name.to_owned(),
                    description: struct_.description.to_owned(),
                    fields: struct_
                        .fields
                        .iter()
                        .map(|(k, v)| {
                            let schema = get_schema(root, &v.schema).unwrap();

                            (
                                k.to_owned(),
                                StructField {
                                    description: v.description.to_owned(),
                                    node: from_schema_and_value(root, schema, &Value::Empty, false),
                                },
                            )
                        })
                        .collect(),
                }),
                modified,
            }
        }
        RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
        RustSchemaKind::Enum(_) => todo!(),
    }
}



fn schema_at<'a>(
    root: &'a RustSchemaRoot,
    data_path: &[DataPathType],
) -> anyhow::Result<&'a RustSchema> {
    let mut schema = get_schema(root, &root.schema)?;

    macro_rules! not_compatible_error {
        ($data:expr, $schema:expr) => {
            bail!("schema {} is not compatible with data {}", $schema, $data)
        };
    }

    for data in data_path {
        match &schema.kind {
            RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Array(array) => todo!(),
            RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
            RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Struct(struct_) => {
                let name = data
                    .as_name()
                    .ok_or(anyhow!("expected name, found indice {}", data))?;

                let field = struct_.fields.get(name).ok_or(anyhow!(
                    "no field named {} in {}",
                    name,
                    struct_.name
                ))?;

                schema = get_schema(&root, &field.schema)?;
            }
            RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
            RustSchemaKind::Enum(_) => todo!(),
            RustSchemaKind::Unit => not_compatible_error!(data, "Unit"),
            RustSchemaKind::Boolean => not_compatible_error!(data, "Boolean"),
            RustSchemaKind::Number(number_kind) => not_compatible_error!(data, "Number"),
            RustSchemaKind::Char => not_compatible_error!(data, "Char"),
            RustSchemaKind::String => not_compatible_error!(data, "String"),
        }
    }

    Ok(schema)
}

fn value_at<'a>(value: &'a Value, data_path: &[DataPathType]) -> &'a Value {
    let mut value = value;

    // todo: rewrite with if_let_guards
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
            (Value::Struct(_, map), DataPathType::Name(name)) => match map.0.get(name) {
                Some(value) => value,
                None => return &Value::Empty,
            },
            (Value::TupleStruct(_, values), DataPathType::Indice(i)) => match values.get(*i) {
                Some(value) => value,
                None => return &Value::Empty,
            },
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
