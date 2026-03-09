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
        #[derive(Default)]
        struct PrevInfo<'a> {
            name: Option<&'a String>,
            description: Option<&'a String>,
        }

        #[instrument(skip_all)]
        fn inner<'a>(
            root: &RustSchemaRoot,
            schema: &'a RustSchema,
            value: &'a Value,
            mut info: PrevInfo<'a>,
        ) -> NodeContainer {
            // debug!("schema = {:#?}\nvalue = {:#?}", schema, value);

            let node = match &schema.kind {
                RustSchemaKind::Unit => todo!(),
                RustSchemaKind::Boolean => todo!(),
                RustSchemaKind::Number(number_kind) => todo!(),
                RustSchemaKind::Char => todo!(),
                RustSchemaKind::String => Node::String(NodeString {
                    value: value.as_str().map(|v| v.to_owned()),
                }),
                RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
                RustSchemaKind::Array(array) => {
                    let value = if let Some(vec) = value.as_array() {
                        if let Some(template) = &array.kind {
                            let template = root.resolve_schema(template).unwrap();

                            Some(
                                vec.iter()
                                    .map(|v| inner(root, template, v, PrevInfo::default()))
                                    .collect(),
                            )
                        } else {
                            Some(vec![])
                        }
                    } else {
                        None
                    };

                    Node::Array(NodeArray {
                        min: array.min,
                        max: array.max,
                        value,
                        has_template: array.kind.is_some(),
                    })
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

                    info = PrevInfo {
                        name: Some(&struct_.name),
                        description: struct_.description.as_ref(),
                    };

                    Node::Struct(NodeStruct {
                        fields: struct_
                            .fields
                            .iter()
                            .map(|(field_name, field)| {
                                let schema = root.resolve_schema(&field.schema).unwrap();

                                let struct_default =
                                    struct_.default.as_ref().map(rust_schema_value_to_value);
                                let field_default =
                                    field.default.as_ref().map(rust_schema_value_to_value);

                                (
                                    field_name.to_owned(),
                                    inner(
                                        root,
                                        schema,
                                        get_struct_field_value(
                                            value,
                                            &struct_default,
                                            &field_default,
                                            field_name,
                                        ),
                                        PrevInfo {
                                            name: Some(field_name),
                                            description: field.description.as_ref(),
                                        },
                                    ),
                                )
                            })
                            .collect(),
                    })
                }
                RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
                RustSchemaKind::Enum(_) => todo!(),
            };

            NodeContainer {
                node,
                modified: false,
                description: info.description.cloned(),
                name: info.name.cloned(),
            }
        }

        inner(root, schema, value, PrevInfo::default())
    }
}
