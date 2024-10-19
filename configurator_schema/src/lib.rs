use bon::builder;
use json::Value;
use schemars::{schema_for, JsonSchema};

#[builder]
pub fn gen_schema2<S: JsonSchema>(
    source_paths: Option<&[&str]>,
    source_home_paths: Option<&[&str]>,
    write_path: Option<&str>,
    format: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let schema = schema_for!(S);

    let mut value = json::value::to_value(&schema)?;

    let obj = value.as_object_mut().expect("object from json schema");

    if let Some(source_paths) = source_paths {
        obj.insert(
            "X_CONFIGURATOR_SOURCE_PATHS".into(),
            Value::String(source_paths.join(";")),
        );
    }

    if let Some(source_home_paths) = source_home_paths {
        obj.insert(
            "X_CONFIGURATOR_SOURCE_HOME_PATH".into(),
            Value::String(source_home_paths.join(";")),
        );
    }

    if let Some(write_path) = write_path {
        obj.insert(
            "X_CONFIGURATOR_WRITE_PATH".into(),
            Value::String(write_path.to_string()),
        );
    }

    if let Some(format) = format {
        obj.insert(
            "X_CONFIGURATOR_FORMAT".into(),
            Value::String(format.to_string()),
        );
    }

    let str = json::to_string_pretty(&value)?;
    Ok(str)
}
