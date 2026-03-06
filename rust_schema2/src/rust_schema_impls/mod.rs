macro_rules! forward_impl {
    (($($impl:tt)+) => $target:ty) => {
        impl $($impl)+ {
            fn is_inline() -> bool {
                <$target as $crate::RustSchemaTrait>::is_inline()
            }

            fn schema_id() -> String {
                <$target as $crate::RustSchemaTrait>::schema_id()
            }

            fn schema(generator: &mut $crate::SchemaGenerator) -> $crate::RustSchema {
                <$target as $crate::RustSchemaTrait>::schema(generator)
            }
        }
    }
}

mod wrappers;
