mod schema;

pub use rust_schema2_derive::*;
pub use schema::*;

pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait RustSchema {
    fn schema(g: &mut Generator);
}

pub fn schema_for<T: RustSchema>() -> RustSchemaRoot {
    let mut g = Generator::new();
    T::schema(&mut g);

    todo!()
}

#[cfg(test)]
mod test {
    use rust_schema2_derive::RustSchema;

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
}
