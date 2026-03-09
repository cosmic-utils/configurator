use derive_more::Unwrap;
use indexmap::IndexMap;
use rust_schema2::{NumberKind, RustSchema, RustSchemaKind, RustSchemaOrRef, RustSchemaRoot};

use crate::{
    generic_value::{F32, F64, Number, Value},
    node::get_schema,
};

#[derive(Debug)]
pub struct NodeContainer {
    pub node: Node,
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
    schema: &RustSchemaOrRef,
    value: &Value,
) -> NodeContainer {
    let schema = get_schema(root, schema).unwrap();

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
                }
            } else {
                NodeContainer {
                    node: Node::String(NodeString {
                        value: None,
                        tampon: String::default(),
                    }),
                }
            }
        }
        RustSchemaKind::Option(rust_schema_or_ref) => todo!(),
        RustSchemaKind::Array(array) => {
            if let Some(vec) = value.as_array() {
                let res = if let Some(template) = &array.kind {
                    vec.iter()
                        .map(|v| from_schema_and_value(root, template, v))
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
                }
            } else {
                NodeContainer {
                    node: Node::Array(NodeArray {
                        min: array.min.clone(),
                        max: array.max.clone(),
                        value: None,
                    }),
                }
            }
        }
        RustSchemaKind::Tuple(rust_schema_or_refs) => todo!(),
        RustSchemaKind::Map(rust_schema_or_ref) => todo!(),
        RustSchemaKind::Struct(struct_) => {
            
        },
        RustSchemaKind::TupleStruct(tuple_struct) => todo!(),
        RustSchemaKind::Enum(_) => todo!(),
    }
}
