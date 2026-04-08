use std::collections::{BTreeMap, HashMap};

use crate::{RustSchema, RustSchemaKind, RustSchemaTrait, SchemaGenerator};

impl<K, V> RustSchemaTrait for BTreeMap<K, V>
where
    V: RustSchemaTrait,
{
    fn schema(generator: &mut SchemaGenerator) -> RustSchema {
        RustSchema {
            kind: RustSchemaKind::Map(generator.schema_for::<V>()),
        }
    }
}

forward_impl!((<K, V: RustSchemaTrait, H> RustSchemaTrait for HashMap<K, V, H>) => BTreeMap<K, V>);
