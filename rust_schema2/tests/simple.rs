use rust_schema2::{RustSchema, schema_for};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// #[test]
// fn testing() {
//     /// Bonsoir
//     #[derive(RustSchema, JsonSchema, Serialize)]
//     #[serde(default)]
//     struct A {
//         /// Bonjour
//         x: i32,
//         y: Option<Box<B>>,
//     }

//     impl Default for A {
//         fn default() -> Self {
//             Self {
//                 x: 3,
//                 y: Some(Box::new(B::default())),
//             }
//         }
//     }

//     #[derive(RustSchema, JsonSchema, Serialize)]
//     #[serde(default)]
//     struct B {
//         x: i32,
//     }

//     impl Default for B {
//         fn default() -> Self {
//             Self {
//                 x: Default::default(),
//             }
//         }
//     }

//     let schema = schema_for::<A>();

//     let json = json::to_string_pretty(&schema).unwrap();

//     println!("{json}");

//     let schema = schemars::schema_for!(A);

//     let json = json::to_string_pretty(&schema).unwrap();

//     println!("");
//     println!("{json}");
// }

// #[test]
// fn testing() {
//     /// Bonsoir
//     #[derive(JsonSchema, Serialize, Deserialize, Debug)]
//     #[serde(default)]
//     struct A(i32, Option<Box<B>>);

//     impl Default for A {
//         fn default() -> Self {
//             Self(3, Some(Box::new(B { x: 11 })))
//         }
//     }

//     #[derive(JsonSchema, Serialize, Deserialize, Debug)]
//     #[serde(default)]
//     struct B {
//         #[serde(default = "a2")]
//         x: i32,
//     }

//     fn a2 () -> i32 {
//         22
//     }

//     impl Default for B {
//         fn default() -> Self {
//             Self {
//                 x: Default::default(),
//             }
//         }
//     }

//     let schema = schemars::schema_for!(A);

//     let json = json::to_string_pretty(&schema).unwrap();

//     println!("");
//     println!("{json}");

//     let default = json::to_string_pretty(&A::default()).unwrap();

//     println!("{default}");

//     let a: A = json::from_str("[3, {}]").unwrap();

//     dbg!(&a);
// }

#[test]
fn testing() {
    /// Bonsoir
    #[derive(JsonSchema, Serialize, Deserialize, Debug)]
    enum A {
        Simple,
        Complex {
            /// Hello
            #[serde(default = "a2")]
            x: i32,
        },

        Tuple(i32, String),
    }

    impl Default for A {
        fn default() -> Self {
            Self::Complex { x: 3 }
        }
    }

    #[derive(JsonSchema, Serialize, Deserialize, Debug)]
    #[serde(default)]
    struct B {
        #[serde(default = "a2")]
        x: i32,
    }

    fn a2() -> i32 {
        22
    }

    impl Default for B {
        fn default() -> Self {
            Self {
                x: Default::default(),
            }
        }
    }

    let schema = schemars::schema_for!(A);

    let json = json::to_string_pretty(&schema).unwrap();

    println!("");
    println!("{json}");

    let default = json::to_string_pretty(&A::default()).unwrap();

    println!("{default}");

    let a: A = json::from_str("{\"Complex\": {}}").unwrap();

    dbg!(&a);
}
