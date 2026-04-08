use crate::{NumberKind, RustSchema, RustSchemaKind, RustSchemaTrait, SchemaGenerator};

macro_rules! number_impl {
    ($type:ty => $variant:ident) => {
        impl RustSchemaTrait for $type {
            fn schema(generator: &mut SchemaGenerator) -> RustSchema {
                RustSchema {
                    kind: RustSchemaKind::Number(NumberKind::$variant),
                }
            }
        }
    };
}

macro_rules! simple_impl {
    ($type:ty => $variant:ident) => {
        impl RustSchemaTrait for $type {
            fn schema(generator: &mut SchemaGenerator) -> RustSchema {
                RustSchema {
                    kind: RustSchemaKind::$variant,
                }
            }
        }
    };
}

number_impl!(u8 => U8);
number_impl!(u16 => U16);
number_impl!(u32 => U32);
number_impl!(u64 => U64);
number_impl!(u128 => U128);
number_impl!(usize => USize);

number_impl!(i8 => I8);
number_impl!(i16 => I16);
number_impl!(i32 => I32);
number_impl!(i64 => I64);
number_impl!(i128 => I128);
number_impl!(isize => ISize);

number_impl!(f32 => F32);
number_impl!(f64 => F64);

simple_impl!(() => Unit);
simple_impl!(char => Char);
simple_impl!(bool => Boolean);
simple_impl!(str => String);
simple_impl!(String => String);
