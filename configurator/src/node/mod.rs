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

#[derive(Debug)]
pub struct NodeContainer {
    pub node: Node,
    pub is_incomplete: bool,
}

#[derive(Debug, Unwrap)]
#[unwrap(ref_mut)]
#[non_exhaustive]
pub enum Node {
    Unit,
    String(NodeString),
    Number(NodeNumber),
    Struct(NodeStruct),
    TupleStruct(NodeTupleStruct),
}

#[derive(Debug)]
pub struct NodeString {
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct NodeNumber {
    pub kind: NumberKind,
    pub value: Option<Number>,
    pub value_string: String,
}

impl NodeNumber {
    pub fn try_parse_from_str(&self, str: &str) -> anyhow::Result<Number> {
        let v = match self.kind {
            NumberKind::U8 => Number::U8(str.parse::<u8>()?),
            NumberKind::U16 => Number::U16(str.parse::<u16>()?),
            NumberKind::U32 => Number::U32(str.parse::<u32>()?),
            NumberKind::U64 => Number::U64(str.parse::<u64>()?),
            NumberKind::U128 => Number::U128(str.parse::<u128>()?),
            NumberKind::USize => Number::USize(str.parse::<usize>()?),
            NumberKind::I8 => Number::I8(str.parse::<i8>()?),
            NumberKind::I16 => Number::I16(str.parse::<i16>()?),
            NumberKind::I32 => Number::I32(str.parse::<i32>()?),
            NumberKind::I64 => Number::I64(str.parse::<i64>()?),
            NumberKind::I128 => Number::I128(str.parse::<i128>()?),
            NumberKind::ISize => Number::ISize(str.parse::<isize>()?),
            NumberKind::F32 => Number::F32(F32(str.parse::<f32>()?)),
            NumberKind::F64 => Number::F64(F64(str.parse::<f64>()?)),
        };

        Ok(v)
    }
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
    pub is_incomplete: bool,
}

#[derive(Debug)]
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
            kind: number_kind.clone(),
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
                            is_incomplete: false,
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

    Ok(NodeContainer {
        node,
        is_incomplete: false,
    })
}

fn value_at<'a>(value: &'a Value, data_path: &[DataPathType]) -> &'a Value {
    let mut value = value;

    // todo: rewrite with if_let_guards
    for data in data_path {
        value = match (value, data) {
            (Value::Option(value), DataPathType::Name(_)) => todo!(),
            (Value::Option(value), DataPathType::Indice(_)) => todo!(),
            (Value::List(values), DataPathType::Name(_)) => todo!(),
            (Value::List(values), DataPathType::Indice(_)) => todo!(),
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
            (Value::NamedTuple(_, values), DataPathType::Indice(i)) => match values.get(*i) {
                Some(value) => value,
                None => return &Value::Empty,
            },
            _ => return &Value::Empty,
        };
    }

    value
}

pub fn apply_value(
    node: &mut NodeContainer,
    value: &Value,
    data_path: &[DataPathType],
    missing: &Missing,
) {
    let value = value_at(value, data_path);

    node.is_incomplete = missing.is_incomplete(Box::new(data_path.iter()));

    match &mut node.node {
        Node::Unit => {}
        Node::String(node_string) => {
            if let Some(str) = value.as_str() {
                node_string.value = Some(str.to_owned());
            }
        }
        Node::Number(node_number) => {
            if let Some(number) = value.as_number() {
                node_number.value = Some(number.clone());
                node_number.value_string = number.to_string();
            }
        }
        Node::Struct(node_struct) => {
            for (name, field) in &mut node_struct.fields {
                let name_data = DataPathType::Name(name.to_owned());
                let full_path = Box::new(data_path.iter().chain(std::iter::once(&name_data)));
                field.is_incomplete = missing.is_incomplete(full_path);
            }
        }
        Node::TupleStruct(node_tuple_struct) => todo!(),
    }
}

pub struct Missing {
    is_missing: bool,
    childs: HashMap<DataPathType, Missing>,
}

impl Missing {
    pub fn add_missing(&mut self, data_path: Vec<DataPathType>) {
        let mut missing = self;

        for data in data_path {
            if !missing.childs.contains_key(&data) {
                missing.childs.insert(
                    data.clone(),
                    Missing {
                        is_missing: false,
                        childs: HashMap::new(),
                    },
                );
            }

            missing = missing.childs.get_mut(&data).unwrap();
        }

        missing.is_missing = true
    }

    pub fn is_incomplete<'a>(
        &'a self,
        data_path: Box<dyn Iterator<Item = &'a DataPathType> + 'a>,
    ) -> bool {
        let mut missing = self;

        for data in data_path {
            match missing.childs.get(data) {
                Some(m) => missing = m,
                None => return false,
            }
        }

        missing.is_missing
    }
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
