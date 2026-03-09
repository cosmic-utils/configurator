use derive_more::Unwrap;
use indexmap::IndexMap;
use rust_schema2::{NumberKind, RustSchema, RustSchemaKind, RustSchemaOrRef, RustSchemaRoot};

use crate::{
    generic_value::{F32, F64, Number, Value},
    node::{get_schema, rust_schema_value_to_value},
};

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
