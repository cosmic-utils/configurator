use crate::{NumberKind, RustSchema, RustSchemaKind, RustSchemaTrait};

impl RustSchemaTrait for i32 {
    fn schema(generator: &mut crate::SchemaGenerator) -> RustSchema {
        RustSchema {
            description: None,
            kind: RustSchemaKind::Number(NumberKind::I32),
            default: None,
        }
    }
}
