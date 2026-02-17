use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};

use serde::de::Error;

use crate::{
    generic_value::{F32, F64, Map, Number, Value},
    node::{Node, NodeContainer},
    providers::write_and_create_parent,
};

fn value_to_ron_value(value: Value) -> ron_value::Value {
    match value {
        Value::Empty => ron_value::Value::Unit,
        Value::Unit => ron_value::Value::Unit,
        Value::Bool(b) => ron_value::Value::Bool(b),
        Value::Char(c) => ron_value::Value::Char(c),
        Value::Number(number) => ron_value::Value::Number(match number {
            Number::I8(v) => ron_value::Number::I8(v),
            Number::I16(v) => ron_value::Number::I16(v),
            Number::I32(v) => ron_value::Number::I32(v),
            Number::I64(v) => ron_value::Number::I64(v),
            Number::I128(v) => ron_value::Number::I128(v),
            Number::U8(v) => ron_value::Number::U8(v),
            Number::U16(v) => ron_value::Number::U16(v),
            Number::U32(v) => ron_value::Number::U32(v),
            Number::U64(v) => ron_value::Number::U64(v),
            Number::U128(v) => ron_value::Number::U128(v),
            Number::F32(v) => ron_value::Number::F32(ron_value::F32(v.0)),
            Number::F64(v) => ron_value::Number::F64(ron_value::F64(v.0)),
            Number::USize(v) => ron_value::Number::U64(v as u64),
            Number::ISize(v) => ron_value::Number::I64(v as i64),
        }),
        Value::String(s) => ron_value::Value::String(s),
        Value::Bytes(items) => ron_value::Value::Bytes(items),
        Value::Option(opt) => {
            ron_value::Value::Option(opt.map(|v| Box::new(value_to_ron_value(*v))))
        }
        Value::List(values) => {
            ron_value::Value::List(values.into_iter().map(value_to_ron_value).collect())
        }
        Value::Map(map) => {
            let mut m = ron_value::Map::new();
            for (k, v) in map.0 {
                m.insert(value_to_ron_value(k), value_to_ron_value(v));
            }
            ron_value::Value::Map(m)
        }
        Value::Tuple(values) => {
            ron_value::Value::Tuple(values.into_iter().map(value_to_ron_value).collect())
        }
        Value::UnitStruct(name) => ron_value::Value::UnitStruct(name),
        Value::Struct(name, map) => {
            let mut m = ron_value::Map::new();
            for (k, v) in map.0 {
                m.insert(k, value_to_ron_value(v));
            }
            ron_value::Value::Struct(name, m)
        }
        Value::NamedTuple(name, values) => {
            ron_value::Value::NamedTuple(name, values.into_iter().map(value_to_ron_value).collect())
        }
    }
}

fn ron_value_to_value(value: ron_value::Value) -> Value {
    match value {
        ron_value::Value::Unit => Value::Empty,
        ron_value::Value::Bool(bool) => Value::Bool(bool),
        ron_value::Value::Char(c) => Value::Char(c),
        ron_value::Value::Number(number) => Value::Number(match number {
            ron_value::Number::I8(v) => Number::I8(v),
            ron_value::Number::I16(v) => Number::I16(v),
            ron_value::Number::I32(v) => Number::I32(v),
            ron_value::Number::I64(v) => Number::I64(v),
            ron_value::Number::I128(v) => Number::I128(v),
            ron_value::Number::U8(v) => Number::U8(v),
            ron_value::Number::U16(v) => Number::U16(v),
            ron_value::Number::U32(v) => Number::U32(v),
            ron_value::Number::U64(v) => Number::U64(v),
            ron_value::Number::U128(v) => Number::U128(v),
            ron_value::Number::F32(v) => Number::F32(F32(v.0)),
            ron_value::Number::F64(v) => Number::F64(F64(v.0)),
        }),
        ron_value::Value::String(s) => Value::String(s),
        ron_value::Value::Bytes(bytes) => Value::Bytes(bytes),
        ron_value::Value::Option(value) => {
            Value::Option(value.map(|v| Box::new(ron_value_to_value(*v))))
        }
        ron_value::Value::List(values) => {
            Value::List(values.into_iter().map(ron_value_to_value).collect())
        }
        ron_value::Value::Map(map) => {
            let mut map2 = Map::new();

            for (key, value) in map {
                map2.0
                    .insert(ron_value_to_value(key), ron_value_to_value(value));
            }

            Value::Map(map2)
        }
        ron_value::Value::Tuple(values) => {
            Value::List(values.into_iter().map(ron_value_to_value).collect())
        }
        ron_value::Value::UnitStruct(name) => Value::UnitStruct(name),
        ron_value::Value::Struct(name, map) => {
            let mut map2 = Map::new();

            for (key, value) in map {
                map2.0.insert(key, ron_value_to_value(value));
            }

            Value::Struct(name, map2)
        }
        ron_value::Value::NamedTuple(name, values) => {
            Value::NamedTuple(name, values.into_iter().map(ron_value_to_value).collect())
        }
    }
}

pub fn read(path: &Path) -> anyhow::Result<Value> {
    let mut map = Map::new();

    for dir_entry in fs::read_dir(path)? {
        let dir_entry = dir_entry?;

        let filename = dir_entry.file_name();

        let filename = filename.to_str().ok_or(anyhow!("no filename"))?;

        let content = fs::read_to_string(dir_entry.path())?;

        debug!("{}", content);

        let value = ron_value::from_str(&content)?;

        let value = ron_value_to_value(value);

        debug!("{:?}", value);

        map.0.insert(filename.to_string(), value);
    }

    // todo: is name in the path variable ?
    Ok(Value::Struct(None, map))
}

pub fn write(path: &Path, value: Value) -> anyhow::Result<()> {
    let ron_value = value_to_ron_value(value);

    let map = if let Value::Struct(_, map) = value {
        map
    } else {
        bail!("initial value is not a struct")
    };

    for (key, value) in map.0 {

        let value = value_to_ron_value(value);


        let content = ron_value::to_string(&value);



        write_and_create_parent(path.join(key), &content)?;
    }


    Ok(())
}
