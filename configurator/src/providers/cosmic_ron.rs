use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};
use figment::{
    Figment, Metadata, Profile, Provider,
    value::{Dict, Empty, Num, Tag, Value},
};
use ron::Map;
use serde::de::Error;

use crate::{
    node::{Node, NodeContainer},
    providers::write_and_create_parent,
};

pub struct CosmicRonProvider {
    path: PathBuf,
}

impl CosmicRonProvider {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }
}

impl Provider for CosmicRonProvider {
    fn metadata(&self) -> figment::Metadata {
        Metadata::named("cosmic ron provider")
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        // dbg!(&map);

        self.data_impl().map_err(figment::Error::custom)
    }
}

impl CosmicRonProvider {
    fn data_impl(
        &self,
    ) -> anyhow::Result<figment::value::Map<figment::Profile, figment::value::Dict>> {
        // let version = {
        //     let mut max: Option<u64> = None;

        //     for dir_entry in fs::read_dir(&self.path)? {
        //         if let Some(filename) = dir_entry?.file_name().to_str() {
        //             if let Some(version) = filename.strip_prefix('v') {
        //                 if let Ok(version) = version.parse::<u64>() {
        //                     max = match max {
        //                         Some(old) => {
        //                             if old < version {
        //                                 Some(version)
        //                             } else {
        //                                 Some(old)
        //                             }
        //                         }
        //                         None => Some(version),
        //                     };
        //                 }
        //             }
        //         }
        //     }
        //     max.ok_or(anyhow!("no version found"))?
        // };

        // let path = self.path.join(format!("v{}", version));

        let mut ron_map = ron::Map::new();

        for dir_entry in fs::read_dir(&self.path)? {
            let dir_entry = dir_entry?;

            let filename = dir_entry.file_name();

            let filename = filename.to_str().ok_or(anyhow!("no filename"))?;

            let content = fs::read_to_string(dir_entry.path())?;

            debug!("{}", content);

            let value: ron::Value = ron::from_str(&content)?;

            debug!("{:?}", value);

            ron_map.insert(ron::Value::String(filename.to_string()), value);
        }

        debug!("{:?}", ron_map);

        let data = Figment::new()
            .merge(figment::providers::Serialized::from(
                ron_map,
                Profile::Default,
            ))
            .data()?;

        Ok(data)
    }
}

fn serialize(path: &Path, value: &Value, schema: &NodeContainer) -> anyhow::Result<()> {
    if let Some(dict) = value.as_dict() {
        for (key, value) in dict {
            match &schema.node {
                Node::Null => todo!(),
                Node::Bool(node_bool) => todo!(),
                Node::String(node_string) => todo!(),
                Node::Number(node_number) => todo!(),
                Node::Object(node_object) => todo!(),
                Node::Enum(node_enum) => todo!(),
                Node::Array(node_array) => todo!(),
                Node::Value(node_value) => todo!(),
                Node::Any => todo!(),
            }

            // write_and_create_parent(path.join(key), &content)?;
        }
    }

    Ok(())
}

fn to_figment_value(value: ron_value::Value) -> Value {
    match value {
        ron_value::Value::Unit => Value::Empty(Tag::Default, Empty::Unit),
        ron_value::Value::Bool(bool) => Value::Bool(Tag::Default, bool),
        ron_value::Value::Char(c) => Value::String(Tag::Default, c.to_string()),
        ron_value::Value::Number(number) => Value::Num(
            Tag::Default,
            match number {
                ron_value::Number::I8(v) => Num::I8(v),
                ron_value::Number::I16(v) => Num::I16(v),
                ron_value::Number::I32(v) => Num::I32(v),
                ron_value::Number::I64(v) => Num::I64(v),
                ron_value::Number::I128(v) => Num::I128(v),
                ron_value::Number::U8(v) => Num::U8(v),
                ron_value::Number::U16(v) => Num::U16(v),
                ron_value::Number::U32(v) => Num::U32(v),
                ron_value::Number::U64(v) => Num::U64(v),
                ron_value::Number::U128(v) => Num::U128(v),
                ron_value::Number::F32(v) => Num::F32(v.0),
                ron_value::Number::F64(v) => Num::F64(v.0),
            },
        ),
        ron_value::Value::String(s) => Value::String(Tag::Default, s),
        ron_value::Value::Bytes(items) => todo!("no bytes implemented"),
        ron_value::Value::Option(value) => match value {
            Some(value) => to_figment_value(*value),
            None => Value::Empty(Tag::Default, Empty::None),
        },
        ron_value::Value::List(values) => Value::Array(
            Tag::Default,
            values.into_iter().map(to_figment_value).collect(),
        ),
        ron_value::Value::Map(map) => {
            let mut map2 = Dict::new();

            for (key, value) in map {
                match key {
                    ron_value::Value::String(key) => {
                        map2.insert(key, to_figment_value(value));
                    }
                    _ => panic!("map with keys other that string are not supported"),
                }
            }

            Value::Dict(Tag::Default, map2)
        }
        ron_value::Value::Tuple(values) => Value::Array(
            Tag::Default,
            values.into_iter().map(to_figment_value).collect(),
        ),
        // todo: rewrite the figment crate :/
        ron_value::Value::UnitStruct(name) => Value::String(Tag::Default, name),
        ron_value::Value::Struct(_name, map) => {
            let mut map2 = Dict::new();

            for (key, value) in map {
                map2.insert(key, to_figment_value(value));
            }

            Value::Dict(Tag::Default, map2)
        }
        ron_value::Value::NamedTuple(_name, values) => Value::Array(
            Tag::Default,
            values.into_iter().map(to_figment_value).collect(),
        ),
    }
}
