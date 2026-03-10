use indexmap::IndexMap;
use rust_schema2::{RustSchema, RustSchemaKind, RustSchemaRoot};

use crate::{
    generic_value::Value,
    node::{Node, NodeArray, NodeContainer, NodeString, NodeStruct, rust_schema_value_to_value},
};

impl NodeContainer {
    #[instrument(skip_all)]
    pub fn from_schema_and_value(
        root: &RustSchemaRoot,
        schema: &RustSchema,
        value: &Value,
    ) -> Self {
        match &schema.kind {
            RustSchemaKind::Unit => todo!(),
            RustSchemaKind::Boolean => todo!(),
            RustSchemaKind::Number(number_kind) => todo!(),
            RustSchemaKind::Char => todo!(),
            RustSchemaKind::String => NodeContainer::from_node(Node::String(NodeString {
                value: value.as_str().map(|v| v.to_owned()),
            })),
            RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Array(array) => {
                let value = if let Some(vec) = value.as_array() {
                    if let Some(template) = &array.template {
                        let template = root.resolve_schema(template).unwrap();

                        Some(
                            vec.iter()
                                .map(|v| {
                                    Self::from_schema_and_value(root, template, v)
                                        .set_is_removable(true)
                                })
                                .collect(),
                        )
                    } else {
                        Some(vec![])
                    }
                } else {
                    None
                };

                NodeContainer::from_node(Node::Array(NodeArray {
                    min: array.min,
                    max: array.max,
                    value,
                    has_template: array.template.is_some(),
                }))
            }
            RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
            RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Struct(struct_) => {
                fn get_struct_field_value<'a>(
                    prev_value: &'a Value,
                    struct_default: &'a Option<Value>,
                    field_default: &'a Option<Value>,
                    field_name: &'a str,
                ) -> &'a Value {
                    if let Some((_, map)) = prev_value.as_struct()
                        && let Some(value) = map.0.get(field_name)
                    {
                        return value;
                    }

                    if let Some(field_default) = field_default {
                        return field_default;
                    }

                    if let Some(struct_default) = struct_default
                        && let Some((_, map)) = struct_default.as_struct()
                        && let Some(value) = map.0.get(field_name)
                    {
                        return value;
                    }

                    &Value::Empty
                }

                fn get_struct_field_default<'a>(
                    struct_default: &'a Option<Value>,
                    field_default: &'a Option<Value>,
                    field_name: &'a str,
                ) -> Option<&'a Value> {
                    if let Some(field_default) = field_default {
                        return Some(field_default);
                    }

                    if let Some(struct_default) = struct_default
                        && let Some((_, map)) = struct_default.as_struct()
                        && let Some(value) = map.0.get(field_name)
                    {
                        return Some(value);
                    }

                    None
                }

                let struct_default = struct_.default.as_ref().map(rust_schema_value_to_value);

                NodeContainer::from_node(Node::Struct(NodeStruct {
                    fields: struct_
                        .fields
                        .iter()
                        .map(|(field_name, field)| {
                            let schema = root.resolve_schema(&field.schema).unwrap();

                            let field_default =
                                field.default.as_ref().map(rust_schema_value_to_value);

                            (
                                field_name.to_owned(),
                                Self::from_schema_and_value(
                                    root,
                                    schema,
                                    get_struct_field_value(
                                        value,
                                        &struct_default,
                                        &field_default,
                                        field_name,
                                    ),
                                )
                                .set_name(field_name.to_owned())
                                .set_description(field.description.to_owned())
                                .set_default(
                                    get_struct_field_default(
                                        &struct_default,
                                        &field_default,
                                        field_name,
                                    )
                                    .cloned(),
                                ),
                            )
                        })
                        .collect(),
                }))
                .set_name(struct_.name.to_owned())
                .set_description(struct_.description.to_owned())
                .set_default(struct_default)
            }
            RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
            RustSchemaKind::Enum(_) => todo!(),
        }
    }
}
