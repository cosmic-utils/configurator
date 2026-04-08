pub use configurator_utils::ConfigFormat;
use json::Value;
use rust_schema2::{RustSchemaTrait, schema_for};

pub use rust_schema2;

#[derive(Clone, Debug, Default)]
pub struct SchemaGenerator {
    source_paths: Vec<String>,
    source_home_path: Option<String>,
    write_path: Option<String>,
    format: Option<ConfigFormat>,
}

impl SchemaGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn source_paths<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<String>,
    {
        self.source_paths = paths.into_iter().map(Into::into).collect();
        self
    }

    pub fn source_home_path(mut self, path: impl Into<String>) -> Self {
        self.source_home_path = Some(path.into());
        self
    }

    pub fn write_path(mut self, path: impl Into<String>) -> Self {
        self.write_path = Some(path.into());
        self
    }

    pub fn format(mut self, format: ConfigFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn generate<T: RustSchemaTrait>(self) -> Result<String, Box<dyn std::error::Error>> {
        let schema = schema_for::<T>();

        // schema.assert_default_no_conflict()?;

        let mut value = json::value::to_value(&schema)?;

        let obj = value.as_object_mut().expect("object from json schema");

        if !self.source_paths.is_empty() {
            obj.insert(
                "X_CONFIGURATOR_SOURCE_PATHS".into(),
                Value::String(self.source_paths.join(";")),
            );
        }

        if let Some(source_home_path) = self.source_home_path {
            obj.insert(
                "X_CONFIGURATOR_SOURCE_HOME_PATH".into(),
                Value::String(source_home_path.to_string()),
            );
        }

        if let Some(write_path) = self.write_path {
            obj.insert(
                "X_CONFIGURATOR_WRITE_PATH".into(),
                Value::String(write_path.to_string()),
            );
        }

        if let Some(format) = self.format {
            obj.insert(
                "X_CONFIGURATOR_FORMAT".into(),
                Value::String(format.to_string()),
            );
        }

        let str = json::to_string_pretty(&value)?;
        Ok(str)
    }
}
