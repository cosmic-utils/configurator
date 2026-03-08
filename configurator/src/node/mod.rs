use std::{borrow::Cow, collections::BTreeMap, fmt::Display, rc::Rc};

use derive_more::derive::Unwrap;
use indexmap::IndexMap;
use light_enum::LightEnum;
use rust_schema2::{RustSchema, RustSchemaKind, RustSchemaOrRef, RustSchemaRoot};

use crate::{
    generic_value::{Map, Number, Value},
    node::data_path::DataPathType,
};

use anyhow::{anyhow, bail};

pub mod data_path;
mod number;
pub use number::{NumberValue, NumberValueLight};
// #[cfg(test)]
// mod tests;

#[derive(Debug, Clone)]
pub struct NodeContainer {
    pub node: Node,
}

#[derive(Debug, Clone, Unwrap)]
#[unwrap(ref_mut)]
#[non_exhaustive]
pub enum Node {
    Unit,
    String(NodeString),
    Number(NodeNumber),
    Struct(NodeStruct),
    TupleStruct(NodeTupleStruct),
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
pub struct NodeStruct {
    pub name: String,
    pub description: Option<String>,
    pub fields: IndexMap<String, StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NodeTupleStruct {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<()>,
}

pub fn get_node(
    root: &RustSchemaRoot,
    data_path: &[DataPathType],
) -> anyhow::Result<NodeContainer> {
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

    let node = match &schema.kind {
        RustSchemaKind::Unit => Node::Unit,
        RustSchemaKind::Boolean => todo!(),
        RustSchemaKind::Number(number_kind) => Node::Number(NodeNumber {
            kind: match number_kind {
                rust_schema2::NumberKind::U8 => NumberValueLight::U8,
                rust_schema2::NumberKind::U16 => NumberValueLight::U16,
                rust_schema2::NumberKind::U32 => NumberValueLight::U32,
                rust_schema2::NumberKind::U64 => NumberValueLight::U64,
                rust_schema2::NumberKind::U128 => NumberValueLight::U128,
                rust_schema2::NumberKind::USize => NumberValueLight::USize,
                rust_schema2::NumberKind::I8 => NumberValueLight::I8,
                rust_schema2::NumberKind::I16 => NumberValueLight::I16,
                rust_schema2::NumberKind::I32 => NumberValueLight::I32,
                rust_schema2::NumberKind::I64 => NumberValueLight::I64,
                rust_schema2::NumberKind::I128 => NumberValueLight::I128,
                rust_schema2::NumberKind::ISize => NumberValueLight::ISize,
                rust_schema2::NumberKind::F32 => NumberValueLight::F32,
                rust_schema2::NumberKind::F64 => NumberValueLight::F64,
            },
            value: None,
            value_string: String::new(),
        }),
        RustSchemaKind::Char => todo!(),
        RustSchemaKind::String => Node::String(NodeString { value: None }),
        RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
        RustSchemaKind::Array(array) => todo!(),
        RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
        RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
        RustSchemaKind::Struct(struct_) => Node::Struct(NodeStruct {
            name: struct_.name.to_owned(),
            description: struct_.description.to_owned(),
            fields: struct_
                .fields
                .iter()
                .map(|(k, v)| {
                    (
                        k.to_owned(),
                        StructField {
                            description: v.description.to_owned(),
                        },
                    )
                })
                .collect(),
        }),
        RustSchemaKind::TupleStruct(tuple_struct) => Node::TupleStruct(NodeTupleStruct {
            name: tuple_struct.name.to_owned(),
            description: tuple_struct.description.to_owned(),
            fields: tuple_struct.fields.iter().map(|_| ()).collect(),
        }),
        RustSchemaKind::Enum(_) => todo!(),
    };

    Ok(NodeContainer { node })
}

pub fn get_value(root: &RustSchemaRoot, initial_value: &Value) -> anyhow::Result<Value> {
    fn inner(
        root: &RustSchemaRoot,
        schema: &RustSchema,
        value: &Value,
        is_default: bool,
    ) -> anyhow::Result<Value> {
        macro_rules! imcomplete {
            ($value:expr, $schema:expr) => {
                bail!("imcomplete value: {:?} {:?}", $value, $schema,)
            };
        }

        macro_rules! not_compatible_error {
            ($expected:expr, $found:expr, $schema:expr) => {
                bail!(
                    "imcompatible value: expected {}, found {:?}. Schema: {:?}",
                    $expected,
                    $found,
                    $schema,
                )
            };
        }

        let value = match &schema.kind {
            RustSchemaKind::Unit => {
                if value.is_unit() {
                    value.clone()
                } else {
                    not_compatible_error!("Unit", value, schema)
                }
            }
            RustSchemaKind::Boolean => {
                if value.as_bool().is_some() {
                    value.clone()
                } else {
                    not_compatible_error!("Boolean", value, schema)
                }
            }
            RustSchemaKind::Number(number_kind) => {
                // todo: convert
                if value.as_number().is_some() {
                    value.clone()
                } else {
                    not_compatible_error!("Number", value, schema)
                }
            }
            RustSchemaKind::Char => todo!(),
            RustSchemaKind::String => {
                if value.as_str().is_some() {
                    value.clone()
                } else {
                    not_compatible_error!("String", value, schema)
                }
            }
            RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Array(array) => todo!(),
            RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
            RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Struct(struct_) => {
                if value.is_empty() {
                    if let Some(struct_default) = &struct_.default {
                        // struct_default can't be empty
                        inner(
                            root,
                            schema,
                            &rust_schema_value_to_value(struct_default),
                            true,
                        )?
                    } else {
                        imcomplete!(value, schema)
                    }
                } else if let Some((name, map)) = value.as_struct() {
                    let mut new_map = Map::new();

                    for (field_name, field) in &struct_.fields {
                        if let Some(field_default) = &field.default {
                            if let Some(field_value) = map.0.get(field_name)
                                && !is_default
                            {
                                new_map.0.insert(
                                    field_name.to_owned(),
                                    inner(
                                        root,
                                        get_schema(root, &field.schema)?,
                                        field_value,
                                        is_default,
                                    )?,
                                );
                            } else {
                                new_map.0.insert(
                                    field_name.to_owned(),
                                    inner(
                                        root,
                                        get_schema(root, &field.schema)?,
                                        &rust_schema_value_to_value(field_default),
                                        true,
                                    )?,
                                );
                            }
                        } else {
                            if let Some(field_value) = map.0.get(field_name) {
                                new_map.0.insert(
                                    field_name.to_owned(),
                                    inner(
                                        root,
                                        get_schema(root, &field.schema)?,
                                        field_value,
                                        is_default,
                                    )?,
                                );
                            } else {
                                imcomplete!(value, schema)
                            }
                        }
                    }

                    Value::Struct(name.to_owned(), new_map)
                } else if let Some(name) = value.as_unit_struct() {
                    value.clone()
                } else {
                    not_compatible_error!("Struct", value, schema)
                }
            }
            RustSchemaKind::TupleStruct(tuple_struct) => {
                if value.is_empty() {
                    if let Some(default) = &tuple_struct.default {
                        // struct_default can't be empty
                        return inner(root, schema, &rust_schema_value_to_value(default), true);
                    }
                }

                if let Some((name, values)) = value.as_named_tuple() {
                    Value::NamedTuple(
                        tuple_struct.name.to_owned(),
                        tuple_struct
                            .fields
                            .iter()
                            .zip(values.iter())
                            .map(|(schema, value)| {
                                let schema = get_schema(&root, &root.schema)?;

                                inner(root, schema, value, is_default)
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    )
                } else {
                    not_compatible_error!("TupleStruct", value, schema)
                }
            }
            RustSchemaKind::Enum(_) => todo!(),
        };

        Ok(value)
    }

    let schema = get_schema(&root, &root.schema)?;

    inner(root, schema, initial_value, false)
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
        rust_schema2::Value::TupleStruct(name, values) => Value::NamedTuple(
            name.to_owned(),
            values.iter().map(rust_schema_value_to_value).collect(),
        ),
        rust_schema2::Value::EnumVariantUnit(_) => todo!(),
        rust_schema2::Value::EnumVariantTuple(_, values) => todo!(),
        rust_schema2::Value::EnumVariantStruct(_, btree_map) => todo!(),
    }
}
