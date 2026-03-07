use std::collections::BTreeMap;

use anyhow::{anyhow, bail};
use rust_schema2::{RustSchema, RustSchemaId, RustSchemaKind, RustSchemaOrRef, RustSchemaRoot};

use crate::{
    generic_value::Value,
    node::{
        Node, NodeArray, NodeArrayTemplate, NodeBool, NodeContainer, NodeEnum, NodeNumber,
        NodeString, NodeValue, NumberValueLight,
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
        RustSchemaKind::Array(array) => todo!(),
        RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
        RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
        RustSchemaKind::Struct(_) => todo!(),
        RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
        RustSchemaKind::Enum(enum_) => NodeContainer::from_node(Node::Enum(NodeEnum::new(
            enum_
                .variants
                .iter()
                .map(|variant| match &variant.kind {
                    rust_schema2::EnumVariantKind::Unit => NodeContainer::from_node(Node::Value(
                        NodeValue::new(Value::String(variant.name.clone())),
                    )),
                    rust_schema2::EnumVariantKind::Tuple(schemas) => {
                        NodeContainer::from_node(Node::Array(NodeArray {
                            values: None,
                            template: NodeArrayTemplate::FirstN(
                                schemas
                                    .iter()
                                    .map(|s| schema_object_to_node("tuple variant", def, s))
                                    .collect()?,
                            ),
                            min: None,
                            max: None,
                        }))
                    }
                    rust_schema2::EnumVariantKind::Struct(btree_map) => todo!(),
                })
                .collect()?,
        )))
        .set_description(enum_.description.clone()),
    };

    Ok(node)
}
