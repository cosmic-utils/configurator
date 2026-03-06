use schemars::{JsonSchema, schema_for};

fn main() {
    #[derive(JsonSchema)]
    struct A {
        x: i32,
        y: Option<Box<B>>,
    }

    #[derive(JsonSchema)]
    struct B {
        x: i32,
    }

    let s = schema_for!(A);
}
