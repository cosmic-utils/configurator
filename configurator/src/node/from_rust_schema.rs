use std::collections::BTreeMap;

use anyhow::{anyhow, bail};
use indexmap::IndexMap;
use rust_schema2::{RustSchema, RustSchemaId, RustSchemaKind, RustSchemaOrRef, RustSchemaRoot};

use crate::{
    generic_value::Value,
    node::{
        Node, NodeArray, NodeArrayTemplate, NodeBool, NodeContainer, NodeEnum, NodeNumber,
        NodeObject, NodeString, NodeValue, NumberValueLight,
    },
};

impl NodeContainer {
    pub fn from_rust_schema(root: &RustSchemaRoot) -> anyhow::Result<NodeContainer> {
        schema_object_to_node("root", &root.definitions, &root.schema)
    }
}

#[instrument(skip_all)]
pub(crate) fn schema_object_to_node(
    from: &str,
    def: &BTreeMap<RustSchemaId, RustSchema>,
    schema: &RustSchemaOrRef,
) -> anyhow::Result<NodeContainer> {
    let schema = match schema {
        RustSchemaOrRef::Ref(ref_) => def.get(ref_).ok_or(anyhow!("unknow ref {ref_}"))?,
        RustSchemaOrRef::Schema(rust_schema) => rust_schema,
    };

    let node = match &schema.kind {
        RustSchemaKind::Unit => NodeContainer::from_node(Node::Unit),
        RustSchemaKind::Boolean => NodeContainer::from_node(Node::Bool(NodeBool::new())),
        RustSchemaKind::Number(number_kind) => {
            NodeContainer::from_node(Node::Number(NodeNumber::new(match number_kind {
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
            })))
        }
        RustSchemaKind::Char => todo!(),
        RustSchemaKind::String => NodeContainer::from_node(Node::String(NodeString::new())),
        RustSchemaKind::Option(schema) => {
            NodeContainer::from_node(Node::Enum(NodeEnum::new(vec![
                NodeContainer::from_node(Node::Unit),
                schema_object_to_node("option", def, schema)?,
            ])))
        }
        RustSchemaKind::Array(array) => NodeContainer::from_node(Node::Array(NodeArray {
            values: None,
            template: NodeArrayTemplate::All(Box::new(schema_object_to_node(
                "array",
                def,
                array.kind.as_ref().unwrap(),
            )?)),
            min: None,
            max: None,
        })),
        RustSchemaKind::Tuple(schemas) => {
            let template = schemas
                .iter()
                .map(|s| schema_object_to_node("tuple", def, s))
                .collect::<Result<Vec<_>, _>>()?;

            NodeContainer::from_node(Node::Array(NodeArray {
                values: None,
                template: NodeArrayTemplate::FirstN(template),
                min: None,
                max: None,
            }))
        }
        RustSchemaKind::Map(schemas) => NodeContainer::from_node(Node::Object(NodeObject {
            nodes: IndexMap::new(),
            template: Some(Box::new(schema_object_to_node("map", def, schemas)?)),
        })),
        RustSchemaKind::Struct(struct_) => {
            let nodes = struct_
                .fields
                .iter()
                .map(|(k, v)| {
                    let value = schema_object_to_node("struct", def, &v.schema)?;

                    Ok((k.to_owned(), value))
                })
                .collect::<Result<IndexMap<String, NodeContainer>, anyhow::Error>>()?;
            NodeContainer::from_node(Node::Object(NodeObject {
                nodes,
                template: None,
            }))
            .set_description(struct_.description.clone())
        }
        RustSchemaKind::TupleStruct(tuple_struct) => {
            let template = tuple_struct
                .fields
                .iter()
                .map(|s| schema_object_to_node("tuple struct", def, s))
                .collect::<Result<Vec<_>, _>>()?;

            NodeContainer::from_node(Node::Array(NodeArray {
                values: None,
                template: NodeArrayTemplate::FirstN(template),
                min: None,
                max: None,
            }))
        }
        RustSchemaKind::Enum(enum_) => {
            let variants = enum_
                .variants
                .iter()
                .map(|variant| -> anyhow::Result<NodeContainer> {
                    match &variant.kind {
                        rust_schema2::EnumVariantKind::Unit => Ok(NodeContainer::from_node(
                            Node::Value(NodeValue::new(Value::String(variant.name.clone()))),
                        )),
                        rust_schema2::EnumVariantKind::Tuple(schemas) => {
                            let template = schemas
                                .iter()
                                .map(|s| schema_object_to_node("tuple variant", def, s))
                                .collect::<Result<Vec<_>, _>>()?;
                            Ok(NodeContainer::from_node(Node::Array(NodeArray {
                                values: None,
                                template: NodeArrayTemplate::FirstN(template),
                                min: None,
                                max: None,
                            })))
                        }
                        rust_schema2::EnumVariantKind::Struct(btree_map) => {
                            let nodes = btree_map
                                .iter()
                                .map(|(k, v)| {
                                    let value =
                                        schema_object_to_node("struct variant", def, &v.schema)?;

                                    Ok((k.to_owned(), value))
                                })
                                .collect::<Result<IndexMap<String, NodeContainer>, anyhow::Error>>(
                                )?;
                            Ok(NodeContainer::from_node(Node::Object(NodeObject {
                                nodes,
                                template: None,
                            })))
                        }
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            NodeContainer::from_node(Node::Enum(NodeEnum::new(variants)))
        }
        .set_description(enum_.description.clone()),
    };

    Ok(node)
}
