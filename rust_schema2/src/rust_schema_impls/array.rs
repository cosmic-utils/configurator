use crate::{Array, RustSchema, RustSchemaKind, RustSchemaTrait, SchemaGenerator};

impl<T> RustSchemaTrait for [T; 0] {
    fn schema(_: &mut SchemaGenerator) -> RustSchema {
        RustSchema {
            kind: RustSchemaKind::Array(Array::empty()),
        }
    }
}

macro_rules! array_impls {
    ($($len:tt)+) => {
        $(
            impl<T: RustSchemaTrait> RustSchemaTrait for [T; $len] {

                fn schema(generator: &mut SchemaGenerator) -> RustSchema {
                    RustSchema {
                        kind: RustSchemaKind::Array(Array {
                            min: Some($len),
                            max: Some($len),
                            template: Some(generator.schema_for::<T>())
                        }),
                    }
                }
            }
        )+
    }
}

array_impls! {
    1  2  3  4  5  6  7  8  9 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}
