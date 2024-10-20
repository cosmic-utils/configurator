use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::app::APPID;

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    last_used_page: Option<String>,
}

#[test]
pub fn gen_schema() {
    let path = Path::new("../configurator/res").join(format!("{}.json", APPID));

    let schema = configurator_schema::gen_schema::<Config>()
        .source_home_paths(&[".config/configurator/configurator.json"])
        .call()
        .unwrap();

    fs::write(path, &schema).unwrap();
}
