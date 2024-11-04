use std::collections::BTreeMap;

use figment::{
    value::{Dict, Empty, Num, Tag, Value},
    Metadata, Profile, Provider,
};
use schemars::schema::{Schema, SchemaObject};

use schemars::_serde_json as json;

pub struct JsonSchemaProvider<'a> {
    schema: &'a SchemaObject,
}

impl<'a> JsonSchemaProvider<'a> {
    pub fn new(schema: &'a SchemaObject) -> Self {
        Self { schema }
    }
}

impl Provider for JsonSchemaProvider<'_> {
    fn metadata(&self) -> figment::Metadata {
        Metadata::named("name")
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        let mut map = figment::value::Map::new();

        let mut dict = Dict::new();

        schema_object(&mut dict, self.schema);

        map.insert(Profile::default(), dict);

        Ok(map)
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

fn schema_object(map: &mut BTreeMap<String, Value>, schema: &SchemaObject) {
    if let Some(schema) = &schema.object {
        for (field_name, schema_field) in &schema.properties {
            match schema_field {
                Schema::Bool(_) => todo!(),
                Schema::Object(schema_field) => {
                    if let Some(metadata) = &schema_field.metadata {
                        if let Some(default) = &metadata.default {
                            let value = json_value_to_figment_value(default);

                            map.insert(field_name.to_string(), value);
                        }
                    }
                }
            }
        }
    }
}
