use rust_schema2::RustSchema;

#[test]
fn testing() {
    #[derive(RustSchema)]
    struct A {
        x: i32,
        y: Option<Box<B>>,
    }

    #[derive(RustSchema)]
    struct B {
        x: i32,
    }

    
    
}
