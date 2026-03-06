macro_rules! forward_impl {
    (($($impl:tt)+) => $target:ty) => {
        impl $($impl)+ {


            fn schema_id() -> Option<String> {
                <$target as $crate::RustSchemaTrait>::schema_id()
            }

            fn schema(generator: &mut $crate::SchemaGenerator) -> $crate::RustSchema {
                <$target as $crate::RustSchemaTrait>::schema(generator)
            }
        }
    }
}

mod core;
mod primitives;
mod wrappers;
