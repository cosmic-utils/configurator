use crate::{RustSchema, RustSchemaKind, RustSchemaTrait, SchemaGenerator};

impl<T: RustSchemaTrait> RustSchemaTrait for Option<T> {
    fn schema(generator: &mut SchemaGenerator) -> RustSchema {
        RustSchema {
            kind: RustSchemaKind::Option(generator.schema_for::<T>()),
        }
    }
}
