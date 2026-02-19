use std::{fmt::Debug, fs, path::Path, str::FromStr};

use configurator_utils::ConfigFormat;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};

use crate::node::NodeContainer;

mod testing1;
mod testing2;

fn get_schema<C: JsonSchema>(name: &str) -> String {
    let config_path = format!("{}/test_configs/{}", env!("CARGO_MANIFEST_DIR"), name);

    configurator_schema::gen_schema::<C>()
        .source_home_path(&config_path)
        .format(ConfigFormat::CosmicRon)
        .call()
        .unwrap()
}

pub fn print_schema<C: JsonSchema>(name: &str) {
    let e = get_schema::<C>(name);

    println!("{}", e);
}

pub fn print_node_container<C: JsonSchema>(name: &str) {
    let content = get_schema::<C>(name);

    let json_value = json::Value::from_str(&content).unwrap();
    let tree = NodeContainer::from_json_schema(&json::from_value(json_value).unwrap());

    println!("{:#?}", tree);
}

pub fn gen_schema<C: JsonSchema>(name: &str) {
    let schema = get_schema::<C>(name);

    let schemas_path = Path::new("test_schemas");

    if !schemas_path.exists() {
        fs::create_dir_all(schemas_path).unwrap();
    }

    let schema_path = schemas_path.join(format!("{}.json", name));

    fs::write(schema_path, &schema).unwrap();
}

pub fn print_json<C: Default + Serialize>() {
    let e = json::to_string_pretty(&C::default()).unwrap();

    println!("{}", e);
}

pub fn print_ron<C: Default + Serialize>() {
    let e = ron::to_string(&C::default()).unwrap();

    println!("{}", e);
}

pub fn from_ron<C: Debug + DeserializeOwned>(ron: &str) {
    let e: C = ron::from_str(ron).unwrap();

    println!("{:?}", e);
}
