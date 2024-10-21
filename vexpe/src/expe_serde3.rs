use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
struct Config {
    hash_map: HashMap<String, Complex>,
    vec: Vec<Complex>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct Complex {
    str: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut hash_map = HashMap::new();

        hash_map.insert("k".into(), Complex { str: "0".into() });

        Self {
            hash_map,
            vec: vec![Complex { str: "0".into() }, Complex { str: "12".into() }],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn deser() {
        let content = fs::read_to_string("test3.json").unwrap();

        let s = serde_json::from_str::<Config>(&content).unwrap();

        dbg!(&s);
    }
}
