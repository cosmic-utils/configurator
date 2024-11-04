use figment::{providers::Format, Provider};
use serde::Serialize;

pub fn backend() -> impl BackEnd {
    json::Json {}
}

pub trait BackEnd {
    fn serialise<V: Serialize>(&self, value: &V) -> anyhow::Result<String>;
    fn extension(&self) -> &'static str;
    fn figment_from(&self, string: &str) -> anyhow::Result<impl Provider> {
        let e = figment::providers::Json::string(string);
        Ok(e)
    }
}

#[cfg(feature = "json")]
pub mod json {
    use serde::Serialize;

    use super::BackEnd;

    pub struct Json;

    impl BackEnd for Json {
        fn serialise<V: Serialize>(&self, value: &V) -> anyhow::Result<String> {
            let str = json::ser::to_string_pretty(value)?;

            Ok(str)
        }

        fn extension(&self) -> &'static str {
            ".json"
        }
    }
}
