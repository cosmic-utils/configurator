use rust_schema2::{RustSchema, schema_for};
use serde::{Deserialize, Serialize};

#[test]
fn test() {
    #[derive(RustSchema, Deserialize, Serialize)]
    #[serde(default)]
    struct A {
        b: B,
    }

    impl Default for A {
        fn default() -> Self {
            Self {
                b: B {
                    s: String::from("hello1"),
                },
            }
        }
    }

    #[derive(RustSchema, Deserialize, Serialize)]
    #[serde(default)]
    struct B {
        s: String,
    }

    impl Default for B {
        fn default() -> Self {
            Self {
                s: Default::default(),
            }
        }
    }

    let schema = schema_for::<A>();

    let res = schema.assert_default_no_conflict();

    if let Err(e) = &res {
        println!("{e}");
    }

    res.unwrap_err();
}

#[test]
fn test2() {
    #[derive(RustSchema, Deserialize, Serialize)]
    struct A {
        #[serde(default)]
        b: B,
    }

    impl Default for A {
        fn default() -> Self {
            Self {
                b: B {
                    s: String::from("hello1"),
                },
            }
        }
    }

    #[derive(RustSchema, Deserialize, Serialize)]
    #[serde(default)]
    struct B {
        s: String,
    }

    impl Default for B {
        fn default() -> Self {
            Self {
                s: String::from("hello1"),
            }
        }
    }

    let schema = schema_for::<A>();

    let res = schema.assert_default_no_conflict();

    if let Err(e) = &res {
        println!("{e}");
    }

    res.unwrap_err();
}

#[test]
fn test3() {
    #[derive(RustSchema, Deserialize, Serialize)]
    #[serde(default)]
    struct A {
        #[serde(default)]
        b: B,
        e: E,
    }

    impl Default for A {
        fn default() -> Self {
            Self {
                b: B {
                    s: String::from("hello1"),
                },
                e: E::Struct {
                    x: 0,
                    y: "bonjour".into(),
                },
            }
        }
    }

    #[derive(RustSchema, Deserialize, Serialize)]
    #[serde(default)]
    struct B {
        s: String,
    }

    impl Default for B {
        fn default() -> Self {
            Self {
                s: String::from("hello1"),
            }
        }
    }

    /// Doc on Enum
    #[derive(RustSchema, Serialize, Deserialize, Debug)]
    enum E {
        /// Doc on variant
        Unit,
        NewType(String),
        Tuple(String, i32),
        Struct {
            /// Doc on field variant
            #[serde(default)]
            x: i32,
            #[serde(default = "bj")]
            y: String,
        },
    }

    fn bj() -> String {
        "bonjour".into()
    }

    let schema = schema_for::<A>();

    let res = schema.assert_default_no_conflict();

    if let Err(e) = &res {
        println!("{e}");
    }

    res.unwrap_err();
}
