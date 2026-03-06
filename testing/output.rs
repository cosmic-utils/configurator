#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use schemars::{JsonSchema, schema_for};
fn main() {
    struct A {
        x: i32,
        y: Option<Box<B>>,
    }
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for A {
            fn schema_name() -> std::string::String {
                "A".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("testing::A")
            }
            fn json_schema(
                generator: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    {
                        schemars::_private::insert_object_property::<
                            i32,
                        >(
                            object_validation,
                            "x",
                            false,
                            false,
                            generator.subschema_for::<i32>(),
                        );
                    }
                    {
                        schemars::_private::insert_object_property::<
                            Option<Box<B>>,
                        >(
                            object_validation,
                            "y",
                            false,
                            false,
                            generator.subschema_for::<Option<Box<B>>>(),
                        );
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        }
    };
    struct B {
        x: i32,
    }
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for B {
            fn schema_name() -> std::string::String {
                "B".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("testing::B")
            }
            fn json_schema(
                generator: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    {
                        schemars::_private::insert_object_property::<
                            i32,
                        >(
                            object_validation,
                            "x",
                            false,
                            false,
                            generator.subschema_for::<i32>(),
                        );
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        }
    };
    let s = ::schemars::gen::SchemaGenerator::default().into_root_schema_for::<A>();
}
