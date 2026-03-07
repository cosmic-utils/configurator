use crate::{NumberKind, RustSchema, RustSchemaKind, RustSchemaTrait, SchemaGenerator};

impl RustSchemaTrait for i32 {
    fn schema(generator: &mut SchemaGenerator) -> RustSchema {
        RustSchema {
            kind: RustSchemaKind::Number(NumberKind::I32),
        }
    }
}
