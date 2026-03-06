use crate::{RustSchema, RustSchemaKind, RustSchemaTrait};

impl<T: RustSchemaTrait> RustSchemaTrait for Option<T> {
    fn schema(generator: &mut crate::SchemaGenerator) -> RustSchema {
        RustSchema {
            description: None,
            kind: RustSchemaKind::Option(generator.schema_for::<T>()),
            default: None,
        }
    }
}
