use core::num;
use std::{borrow::Cow, collections::BTreeMap};

use figment::value::{Empty, Num, Tag};
use json::value::Index;
use schemars::schema::{
    InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec, SubschemaValidation,
};

use super::*;

impl NodeContainer {
    pub fn from_json_schema2(schema: &RootSchema) -> Self {
        // dbg!(&schema);

        // dbg!(&schema.definitions);

        // dbg!(&tree);
        schema_object_to_node2("root", &schema.definitions, &schema.schema)
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
fn schema_object_to_node2(
    from: &str,
    def: &schemars::Map<String, Schema>,
    schema_object: &SchemaObject,
    source: Option<NodeContainer>,
) -> Option<NodeContainer> {
    info!("enter function from {from}");
    dbg!(&schema_object);

    match &schema_object.instance_type {
        Some(instance_type) => {
            
        },
        None => {

            todo!()
        },
    }
}

trait ToSchemaObject {
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

fn json_value_to_figment_value(json_value: &json::Value) -> Value {
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
