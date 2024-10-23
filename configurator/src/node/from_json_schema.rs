use core::num;
use std::{borrow::Cow, collections::BTreeMap};

use figment::value::{Empty, Num, Tag};
use json::value::Index;
use schemars::schema::{
    InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec, SubschemaValidation,
};

use super::*;

impl NodeContainer {
    pub fn from_json_schema(schema: &RootSchema) -> Self {
        // dbg!(&schema);

        // dbg!(&schema.definitions);

        // dbg!(&tree);
        schema_object_to_node("root", &schema.definitions, &schema.schema)
    }
}

fn is_option(vec: &[InstanceType]) -> Option<&InstanceType> {
    if vec.len() == 2 {
        if vec[0] == InstanceType::Null {
            Some(&vec[1])
        } else if vec[1] == InstanceType::Null {
            Some(&vec[0])
        } else {
            None
        }
    } else {
        None
    }
}

// #[tracing::instrument]
pub(crate) fn schema_object_to_node(
    from: &str,
    def: &schemars::Map<String, Schema>,
    schema_object: &SchemaObject,
) -> NodeContainer {
    // info!("enter function from {from}");
    // dbg!(&schema_object);

    let metadata = &schema_object.metadata;

    if let Some(obj) = &schema_object.object {
        let mut nodes = IndexMap::new();

        for (name, type_definition) in &obj.properties {
            let node = schema_object_to_node("object", def, &type_definition.to_object());
            nodes.insert(name.clone(), node);
        }

        let additional_properties = if !obj.properties.is_empty() {
            None
        } else {
            obj.additional_properties
                .as_ref()
                .map(|additional_properties| {
                    schema_object_to_node("object", def, &additional_properties.to_object())
                })
        };

        return NodeContainer::from_metadata(
            Node::Object(NodeObject::new(nodes, additional_properties)),
            metadata,
        );
    }

    if let Some(enum_values) = &schema_object.enum_values {
        // dbg!(schema_object);
        // dbg!(&enum_values);

        return if enum_values.len() == 1 {
            NodeContainer::from_metadata(
                Node::Value(NodeValue::new(enum_values[0].clone())),
                metadata,
            )
        } else {
            let mut nodes = Vec::new();

            for value in enum_values {
                nodes.push(NodeContainer::from_metadata(
                    Node::Value(NodeValue::new(value.clone())),
                    metadata,
                ));
            }

            NodeContainer::from_metadata(Node::Enum(NodeEnum::new(nodes)), metadata)
        };
    }

    if let Some(array) = &schema_object.array {
        let node = match &array.items {
            Some(single_or_vec) => match single_or_vec {
                SingleOrVec::Single(schema) => {
                    let node = schema_object_to_node("array single", def, &schema.to_object());
                    node
                }
                SingleOrVec::Vec(vec) => todo!(),
            },
            None => todo!(),
        };
        return NodeContainer::from_metadata(Node::Array(NodeArray::new(node)), metadata);
    }

    if let Some(single_or_vec) = &schema_object.instance_type {
        fn instance_type_to_node(instance_type: &InstanceType, format: Option<&String>) -> Node {
            match *instance_type {
                InstanceType::Null => Node::Null,
                InstanceType::Boolean => Node::Bool(NodeBool::new()),
                InstanceType::Object => Node::Object(NodeObject::new(IndexMap::new(), None)),
                InstanceType::Array => todo!(),
                InstanceType::Number => Node::Number(NodeNumber::new(
                    format
                        .and_then(|s| NumberValue::kind_from_str(s))
                        .unwrap_or(NumberValueLight::F64),
                )),
                InstanceType::String => Node::String(NodeString::new()),
                InstanceType::Integer => Node::Number(NodeNumber::new(
                    format
                        .and_then(|s| NumberValue::kind_from_str(s))
                        .unwrap_or(NumberValueLight::I128),
                )),
            }
        }

        return match single_or_vec {
            SingleOrVec::Single(instance_type) => NodeContainer::from_metadata(
                instance_type_to_node(instance_type, schema_object.format.as_ref()),
                metadata,
            ),
            SingleOrVec::Vec(vec) => {
                let nodes = vec
                    .iter()
                    .map(|instance_type| {
                        // xxx: why do we not pass metadata here ?

                        NodeContainer::from_metadata(
                            instance_type_to_node(instance_type, schema_object.format.as_ref()),
                            &None,
                        )
                    })
                    .collect();
                NodeContainer::from_metadata(Node::Enum(NodeEnum::new(nodes)), metadata)
            }
        };
    }

    if let Some(subschemas) = &schema_object.subschemas {
        if let Some(all_of) = &subschemas.all_of {
            let mut nodes = Vec::new();

            for schema in all_of {
                let node = schema_object_to_node("all_of", def, &schema.to_object());
                nodes.push(node);
            }

            return if nodes.len() > 1 {
                todo!()
            } else {
                nodes.remove(0).set_metadata(metadata)
            };
        }

        if let Some(one_of) = &subschemas.one_of {
            let mut nodes = Vec::new();
            for schema in one_of {
                let node = schema_object_to_node("one_of", def, &schema.to_object());

                // dbg!(&node);

                nodes.push(node);
            }

            return NodeContainer::from_metadata(Node::Enum(NodeEnum::new(nodes)), metadata);
        }
    }

    if let Some(definition) = &schema_object.reference {
        // dbg!(&schema_object);
        // dbg!(&root_schema.definitions);

        if let Some(definition) = definition.strip_prefix("#/definitions/") {
            let schema = def.get(definition).unwrap();

            return schema_object_to_node("definition", def, &schema.to_object());
        }
    }

    dbg!(&schema_object);
    panic!()
}

pub trait ToSchemaObject {
    fn to_object(&self) -> Cow<'_, SchemaObject>;
}

impl ToSchemaObject for Schema {
    fn to_object(&self) -> Cow<'_, SchemaObject> {
        match self {
            Schema::Object(o) => Cow::Borrowed(o),
            Schema::Bool(true) => Cow::Owned(SchemaObject::default()),
            Schema::Bool(false) => Cow::Owned(SchemaObject {
                subschemas: Some(Box::new(SubschemaValidation {
                    not: Some(Schema::Object(Default::default()).into()),
                    ..Default::default()
                })),
                ..Default::default()
            }),
        }
    }
}

pub fn json_value_to_figment_value(json_value: &json::Value) -> Value {
    match json_value {
        json::Value::Null => Value::Empty(Tag::Default, Empty::None),
        json::Value::Bool(value) => Value::Bool(Tag::Default, *value),
        json::Value::Number(number) => {
            let num = if let Some(n) = number.as_u64() {
                Num::U64(n)
            } else if let Some(n) = number.as_i64() {
                Num::I64(n)
            } else if let Some(n) = number.as_f64() {
                Num::F64(n)
            } else {
                panic!("not a valid number")
            };

            Value::Num(Tag::Default, num)
        }
        json::Value::String(str) => Value::String(Tag::Default, str.clone()),
        json::Value::Array(vec) => {
            let array = vec.iter().map(json_value_to_figment_value).collect();

            Value::Array(Tag::Default, array)
        }
        json::Value::Object(fields) => {
            let dict = fields
                .iter()
                .map(|(name, value)| (name.clone(), json_value_to_figment_value(value)))
                .collect();

            Value::Dict(Tag::Default, dict)
        }
    }
}
