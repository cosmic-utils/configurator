use std::{fs, path::Path};

use figment::{providers, Figment, Profile};
use schemars::JsonSchema;
use serde::Serialize;

mod testing1;
mod testing2;

fn get_schema<C: JsonSchema>(name: &str) -> String {
    let config_path = format!("{}/test_configs/{}.json", env!("CARGO_MANIFEST_DIR"), name);

    configurator_schema::gen_schema::<C>()
        .source_home_paths(&[&config_path])
        .call()
        .unwrap()
}

pub fn print_schema<C: JsonSchema>(name: &str) {
    let e = get_schema::<C>(name);

    print!("{}", e);
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

pub fn print_default_figment<C: Default + Serialize>() {
    let figment =
        Figment::new().merge(providers::Serialized::from(&C::default(), Profile::Default));

    dbg!(&figment);
}

pub fn print_json<C: Default + Serialize>() {
    let e = json::to_string_pretty(&C::default()).unwrap();

    print!("{}", e);
}

pub fn print_ron<C: Default + Serialize>() {
    let e = ron::to_string(&C::default()).unwrap();

    print!("{}", e);
}
