use indexmap::IndexMap;
use rust_schema2::{RustSchema, RustSchemaKind, RustSchemaRoot};

use crate::{
    generic_value::Value,
    node::{
        Node, NodeArray, NodeContainer, NodeString, NodeStruct, StructField,
        rust_schema_value_to_value,
    },
};

impl NodeContainer {
    #[instrument(skip_all)]
    pub fn from_schema_and_value(
        root: &RustSchemaRoot,
        schema: &RustSchema,
        value: &Value,
    ) -> Self {
        // debug!("schema = {:#?}\nvalue = {:#?}", schema, value);

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
                        }),
                        modified: false,
                    }
                } else {
                    NodeContainer {
                        node: Node::String(NodeString { value: None }),
                        modified: false,
                    }
                }
            }
            RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
            RustSchemaKind::Array(array) => {
                if let Some(vec) = value.as_array() {
                    let res = if let Some(template) = &array.kind {
                        let template = root.resolve_schema(template).unwrap();

                        vec.iter()
                            .map(|v| Self::from_schema_and_value(root, template, v))
                            .collect()
                    } else {
                        vec![]
                    };

                    NodeContainer {
                        node: Node::Array(NodeArray {
                            min: array.min,
                            max: array.max,
                            value: Some(res),
                        }),
                        modified: false,
                    }
                } else {
                    NodeContainer {
                        node: Node::Array(NodeArray {
                            min: array.min,
                            max: array.max,
                            value: None,
                        }),
                        modified: false,
                    }
                }
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

                NodeContainer {
                    node: Node::Struct(NodeStruct {
                        name: struct_.name.to_owned(),
                        description: struct_.description.to_owned(),
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
                                    StructField {
                                        description: field.description.to_owned(),
                                        node: Self::from_schema_and_value(
                                            root,
                                            schema,
                                            get_struct_field_value(
                                                value,
                                                &struct_default,
                                                &field_default,
                                                field_name,
                                            ),
                                        ),
                                    },
                                )
                            })
                            .collect(),
                    }),
                    modified: false,
                }
            }
            RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
            RustSchemaKind::Enum(_) => todo!(),
        }
    }
}
