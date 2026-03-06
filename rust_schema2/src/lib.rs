mod schema;

use std::borrow::Cow;

pub use rust_schema2_derive::*;
pub use schema::*;

pub struct Generator {

}

impl Generator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn schema_for<T: RustSchemaTrait>(&mut self) -> RustSchemaRoot {

        // T::schema(generator)

        // RustSchemaRoot {
        //     schema: (),
        //     definitions: (),
        // }

        todo!()
    }
}

pub trait RustSchemaTrait {
    #[must_use]
    fn inline_schema() -> bool {
        true
    }

    #[must_use]
    fn schema_name() -> Cow<'static, str>;

    fn schema(generator: &mut Generator) -> RustSchema;
}

pub fn schema_for<T: RustSchemaTrait>() -> RustSchemaRoot {
    let mut g = Generator::new();
    g.schema_for::<T>()
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
