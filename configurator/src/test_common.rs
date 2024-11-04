use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct Complex {
    x: String,
    y: i32,
}

impl Default for Complex {
    fn default() -> Self {
        Self {
            x: "default of complex mgl".into(),
            y: 10,
        }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct Rec {
    x: String,
    y: Option<Box<Rec>>,
}

impl Default for Rec {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Some(Box::new(Rec {
                x: "nested2".into(),
                y: Some(Box::new(Rec {
                    x: "nested3".into(),
                    y: None,
                })),
            })),
        }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum EnumComplex {
    A,
    B(i32),
    C(Complex),
    D { a: i32, b: Complex },
}

impl Default for EnumComplex {
    fn default() -> Self {
        Self::C(Complex {
            x: "hello".into(),
            y: 1,
        })
    }
}

impl Default for TestVeryComplex {
    fn default() -> Self {
        let mut x = HashMap::new();

        x.insert("hello".into(), Complex::default());

        let mut v = (vec![], HashMap::new());

        v.0.push(EnumComplex::A);

        v.0.push(EnumComplex::D {
            a: 5,
            b: Complex {
                x: "bis".into(),
                y: 123,
            },
        });

        v.1.insert("lol".into(), EnumComplex::default());

        Self {
            x,
            y: Default::default(),
            v,
        }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestBool {
    x: bool,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestString {
    x: String,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestNumber {
    x: i32,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestFloat {
    x: f32,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
pub enum EnumSimple {
    #[default]
    A,
    B,
    C,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestEnumSimple {
    x: EnumSimple,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestEnumComplex {
    x: EnumComplex,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestOption {
    x: Option<String>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestOptionComplex {
    x: Option<Complex>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestTuple {
    x: (Option<Complex>, i32, String),
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestVec {
    x: Vec<Complex>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TestHashMap {
    x: HashMap<String, Complex>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct TestVeryComplex {
    x: HashMap<String, Complex>,
    y: EnumComplex,
    v: (Vec<EnumComplex>, HashMap<String, EnumComplex>),
}
