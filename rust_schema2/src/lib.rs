mod schema;

use std::{borrow::Cow, collections::BTreeMap};

pub use rust_schema2_derive::*;
pub use schema::*;


mod rust_schema_impls;

pub struct SchemaGenerator {
    definitions: BTreeMap<RustSchemaId, Option<RustSchema>>,
}

impl SchemaGenerator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn schema_for<T: RustSchemaTrait>(&mut self) -> RustSchemaOrRef {
        if T::is_inline() {
            RustSchemaOrRef::schema(T::schema(self))
        } else {
            let id = T::schema_id();

            if !self.definitions.contains_key(&id) {
                self.definitions.insert(id.clone(), None);
                let schema = T::schema(self);
                self.definitions.insert(id.clone(), Some(schema));
            }

            RustSchemaOrRef::ref_(id)
        }
    }

    pub fn into_schema_root(self, root: RustSchemaOrRef) -> RustSchemaRoot {
        RustSchemaRoot {
            schema: root,
            definitions: self
                .definitions
                .into_iter()
                .map(|(k, v)| (k, v.unwrap()))
                .collect(),
        }
    }
}

pub trait RustSchemaTrait {
    #[must_use]
    fn is_inline() -> bool {
        true
    }

    #[must_use]
    fn schema_id() -> RustSchemaId;

    #[must_use]
    fn schema(generator: &mut SchemaGenerator) -> RustSchema;
}

pub fn schema_for<T: RustSchemaTrait>() -> RustSchemaRoot {
    let mut g = SchemaGenerator::new();
    let root = g.schema_for::<T>();
    g.into_schema_root(root)
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
