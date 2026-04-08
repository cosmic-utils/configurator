use std::collections::BTreeMap;

use crate::{RustSchema, RustSchemaId, RustSchemaOrRef, RustSchemaRoot};

pub struct SchemaGenerator {
    definitions: BTreeMap<RustSchemaId, Option<RustSchema>>,
}

impl SchemaGenerator {
    #[must_use]
    fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
        }
    }

    pub fn schema_for<T: RustSchemaTrait>(&mut self) -> RustSchemaOrRef {
        if let Some(id) = T::schema_id() {
            if !self.definitions.contains_key(&id) {
                self.definitions.insert(id.clone(), None);
                let schema = T::schema(self);
                self.definitions.insert(id.clone(), Some(schema));
            }

            RustSchemaOrRef::ref_(id)
        } else {
            RustSchemaOrRef::schema(T::schema(self))
        }
    }

    fn into_schema_root(self, root: RustSchemaOrRef) -> RustSchemaRoot {
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
    fn schema_id() -> Option<RustSchemaId> {
        None
    }

    #[must_use]
    fn schema(generator: &mut SchemaGenerator) -> RustSchema;
}

pub fn schema_for<T: RustSchemaTrait>() -> RustSchemaRoot {
    let mut g = SchemaGenerator::new();
    let root = g.schema_for::<T>();
    g.into_schema_root(root)
}
