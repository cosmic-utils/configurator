use core::cmp::Ordering;
use std::fmt::Display;

use facet::Facet;

#[derive(Facet, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    F32(F32),
    F64(F64),
}

macro_rules! float_ty {
    ($ty:ident($float:ty)) => {
        #[derive(Facet, Copy, Clone, Debug)]
        pub struct $ty(pub $float);

        impl $ty {
            #[must_use]
            pub fn new(v: $float) -> Self {
                Self(v)
            }
        }

        impl From<$float> for $ty {
            fn from(v: $float) -> Self {
                Self::new(v)
            }
        }

        impl Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other).is_eq()
            }
        }

        impl Eq for $ty {}

        impl PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $ty {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.total_cmp(&other.0)
            }
        }
    };
}

float_ty! { F32(f32) }
float_ty! { F64(f64) }

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U8(v) => write!(f, "{v}"),
            Number::U16(v) => write!(f, "{v}"),
            Number::U32(v) => write!(f, "{v}"),
            Number::U64(v) => write!(f, "{v}"),
            Number::U128(v) => write!(f, "{v}"),
            Number::USize(v) => write!(f, "{v}"),
            Number::I8(v) => write!(f, "{v}"),
            Number::I16(v) => write!(f, "{v}"),
            Number::I32(v) => write!(f, "{v}"),
            Number::I64(v) => write!(f, "{v}"),
            Number::I128(v) => write!(f, "{v}"),
            Number::ISize(v) => write!(f, "{v}"),
            Number::F32(v) => write!(f, "{v}"),
            Number::F64(v) => write!(f, "{v}"),
        }
    }
}

impl Number {
    pub fn as_u128(&self) -> Option<u128> {
        match *self {
            Number::U8(v) => Some(v.into()),
            Number::U16(v) => Some(v.into()),
            Number::U32(v) => Some(v.into()),
            Number::U64(v) => Some(v.into()),
            Number::U128(v) => Some(v),
            Number::USize(v) => v.try_into().ok(),
            Number::I8(v) => v.try_into().ok(),
            Number::I16(v) => v.try_into().ok(),
            Number::I32(v) => v.try_into().ok(),
            Number::I64(v) => v.try_into().ok(),
            Number::I128(v) => v.try_into().ok(),
            Number::ISize(v) => v.try_into().ok(),
            Number::F32(_) => None,
            Number::F64(_) => None,
        }
    }

    pub fn as_i128(&self) -> Option<i128> {
        match *self {
            Number::U8(v) => Some(v.into()),
            Number::U16(v) => Some(v.into()),
            Number::U32(v) => Some(v.into()),
            Number::U64(v) => Some(v.into()),
            Number::U128(v) => v.try_into().ok(),
            Number::USize(v) => v.try_into().ok(),
            Number::I8(v) => Some(v.into()),
            Number::I16(v) => Some(v.into()),
            Number::I32(v) => Some(v.into()),
            Number::I64(v) => Some(v.into()),
            Number::I128(v) => Some(v),
            Number::ISize(v) => v.try_into().ok(),
            Number::F32(_) => None,
            Number::F64(_) => None,
        }
    }

    /// Returns the [`f64`] representation of the [`Number`] regardless of
    /// whether the number is stored as a float or integer.
    pub fn as_f64(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        match *self {
            Number::U8(v) => v.into(),
            Number::U16(v) => v.into(),
            Number::U32(v) => v.into(),
            Number::I8(v) => v.into(),
            Number::I16(v) => v.into(),
            Number::I32(v) => v.into(),
            Number::F32(v) => v.0.into(),
            Number::F64(v) => v.0,
            Number::U64(v) => v as f64,
            Number::U128(v) => v as f64,
            Number::USize(v) => v as f64,
            Number::I64(v) => v as f64,
            Number::I128(v) => v as f64,
            Number::ISize(v) => v as f64,
        }
    }
}

macro_rules! number_from_impl {
    (Number::$variant:ident($wrap:ident($ty:ty))) => {
        impl From<$ty> for Number {
            fn from(v: $ty) -> Number {
                Number::$variant($wrap(v))
            }
        }
    };
    (Number::$variant:ident($ty:ty)) => {
        impl From<$ty> for Number {
            fn from(v: $ty) -> Number {
                Number::$variant(v)
            }
        }
    };
}

number_from_impl! { Number::I8(i8) }
number_from_impl! { Number::I16(i16) }
number_from_impl! { Number::I32(i32) }
number_from_impl! { Number::I64(i64) }
number_from_impl! { Number::I128(i128) }
number_from_impl! { Number::U8(u8) }
number_from_impl! { Number::U16(u16) }
number_from_impl! { Number::U32(u32) }
number_from_impl! { Number::U64(u64) }
number_from_impl! { Number::U128(u128) }
number_from_impl! { Number::F32(F32(f32)) }
number_from_impl! { Number::F64(F64(f64)) }
