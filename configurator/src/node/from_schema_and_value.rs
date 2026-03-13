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
        default: &Value,
    ) -> Self {
        match &schema.kind {
            RustSchemaKind::Unit => todo!(),
            RustSchemaKind::Boolean => todo!(),
            RustSchemaKind::Number(number_kind) => todo!(),
            RustSchemaKind::Char => todo!(),
            RustSchemaKind::String => NodeContainer::from_node(Node::String(NodeString {
                value: value.as_str().map(|v| v.to_owned()),
            }))
            .set_default(default.clone()),
            RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Array(array) => {
                let value = if let Some(vec) = value.as_array() {
                    if let Some(template) = &array.template {
                        let template = root.resolve_schema(template).unwrap();

                        Some(
                            vec.iter()
                                .map(|v| {
                                    Self::from_schema_and_value(root, template, v, &Value::Empty)
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
                .set_default(default.clone())
            }
            RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
            RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Struct(struct_) => {
                fn get_struct_field_value<'a>(
                    prev_default: &'a Value,
                    struct_default: &'a Value,
                    field_default: &'a Value,
                    field_name: &'a str,
                ) -> &'a Value {
                    let prev_default = if let Some((_, map)) = prev_default.as_struct()
                        && let Some(value) = map.0.get(field_name)
                    {
                        value
                    } else {
                        &Value::Empty
                    };

                    let struct_default = if let Some((_, map)) = struct_default.as_struct()
                        && let Some(value) = map.0.get(field_name)
                    {
                        value
                    } else {
                        &Value::Empty
                    };

                    prev_default
                        .if_not_empty(field_default)
                        .if_not_empty(struct_default)
                }

                let struct_default = struct_
                    .default
                    .as_ref()
                    .map(rust_schema_value_to_value)
                    .unwrap_or(Value::Empty);

                NodeContainer::from_node(Node::Struct(NodeStruct {
                    fields: struct_
                        .fields
                        .iter()
                        .map(|(field_name, field)| {
                            let schema = root.resolve_schema(&field.schema).unwrap();

                            let field_default = field
                                .default
                                .as_ref()
                                .map(rust_schema_value_to_value)
                                .unwrap_or(Value::Empty);

                            let final_field_default = get_struct_field_value(
                                &default,
                                &struct_default,
                                &field_default,
                                field_name,
                            );

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
                                    final_field_default,
                                )
                                .set_name(Some(field_name.to_owned()))
                                .set_description(field.description.to_owned())
                                .set_default(final_field_default.clone()),
                            )
                        })
                        .collect(),
                }))
                .set_name(Some(struct_.name.to_owned()))
                .set_description(struct_.description.to_owned())
                .set_default(default.if_not_empty(&struct_default).clone())
            }
            RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
            RustSchemaKind::Enum(_) => todo!(),
        }
    }
}
