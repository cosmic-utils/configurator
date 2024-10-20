use std::{collections::BTreeMap, fs, panic::catch_unwind, path::PathBuf};

use figment::{providers, Figment, Profile};
use schemars::schema::Schema;
use serde::Deserialize;

use crate::node::from_json_schema::{schema_object_to_node, ToSchemaObject};

fn test_path() -> PathBuf {
    PathBuf::from("../JSON-Schema-Test-Suite/tests/draft7")
}

#[derive(Debug, Deserialize)]
struct TestGroup {
    description: String,
    schema: Schema,
    tests: Vec<Test>,
}

#[derive(Debug, Deserialize)]
struct Test {
    description: String,
    data: json::Value,
    valid: bool,
}

type Tests = Vec<TestGroup>;

#[test]
fn test_all_suite() {
    let mut nb_of_error_deserialization = 0;
    let mut nb_parsed_schema = 0;

    let mut nb_parsed_test = 0;

    let mut nb_of_passed_schema = 0;
    let mut nb_of_passed_test = 0;

    let mut tests = Vec::new();

    for dir_entry in fs::read_dir(test_path()).unwrap() {
        let dir_entry = dir_entry.unwrap();

        let path = dir_entry.path();

        match dir_entry.metadata().unwrap().is_file() {
            true => {
                let content = fs::read_to_string(&path).unwrap();

                match json::from_str::<Tests>(&content) {
                    Ok(test_groups) => {
                        nb_parsed_test += test_groups
                            .iter()
                            .map(|e| e.tests.len() as i32)
                            .sum::<i32>();

                        nb_parsed_schema += test_groups.len();
                        tests.push(test_groups);
                    }
                    Err(e) => {
                        nb_of_error_deserialization += 1;
                        eprintln!("err {e}: {}", path.display());
                    }
                }
            }
            false => {
                // todo
            }
        }
    }

    for test_groups in &tests {
        for test_group in test_groups {
            let tree = match catch_unwind(|| {
                schema_object_to_node(
                    "test",
                    &schemars::Map::new(),
                    &test_group.schema.to_object(),
                )
            }) {
                Ok(n) => {
                    nb_of_passed_schema += 1;
                    n
                }
                Err(err) => {
                    continue;
                }
            };

            for test in &test_group.tests {
                match catch_unwind(|| {
                    let config = Figment::new()
                        .merge(providers::Serialized::from(&test.data, Profile::Default));

                    let mut tree = tree.clone();

                    if tree.apply_figment(&config).is_err() {
                        false
                    } else {
                        tree.is_valid()
                    }
                }) {
                    Ok(is_valid) => {
                        if is_valid == test.valid {
                            nb_of_passed_test += 1;
                        }
                    }
                    Err(err) => {
                        continue;
                    }
                };
            }
        }
    }

    println!("nb_of_error_deserialization: {nb_of_error_deserialization}");
    println!("nb_parsed_schema: {nb_parsed_schema}");
    println!("nb_parsed_test: {nb_parsed_test}");
    println!("nb_of_passed_schema: {nb_of_passed_schema}");
    println!("nb_of_passed_test: {nb_of_passed_test}");
}
