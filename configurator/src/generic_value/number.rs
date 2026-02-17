use core::{
    cmp::{Eq, Ordering},
    hash::{Hash, Hasher},
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
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
        #[doc = concat!(
                    "A wrapper for [`", stringify!($float), "`], which implements [`Eq`], ",
                    "[`Hash`] and [`Ord`] using [`", stringify!($float), "::total_cmp`] ",
                    "for a total order comparison",
                )]
        #[derive(Copy, Clone, Debug)] // GRCOV_EXCL_LINE
        pub struct $ty(pub $float);

        impl $ty {
            #[doc = concat!("Construct a new [`", stringify!($ty), "`].")]
            #[must_use]
            pub fn new(v: $float) -> Self {
                Self(v)
            }

            #[doc = concat!("Returns the wrapped [`", stringify!($float), "`].")]
            #[must_use]
            pub fn get(self) -> $float {
                self.0
            }
        }

        impl From<$float> for $ty {
            fn from(v: $float) -> Self {
                Self::new(v)
            }
        }

        /// Partial equality comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// See the [`Ord`] implementation.
        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other).is_eq()
            }
        }

        /// Equality comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// See the [`Ord`] implementation.
        impl Eq for $ty {}

        impl Hash for $ty {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        /// Partial ordering comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// See the [`Ord`] implementation.
        impl PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        /// Ordering comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// ```
        #[doc = concat!("use ron_value::", stringify!($ty), ";")]
        #[doc = concat!(
                    "assert!(", stringify!($ty), "::new(", stringify!($float), "::NAN) > ",
                    stringify!($ty), "::new(", stringify!($float), "::INFINITY));",
                )]
        #[doc = concat!(
                    "assert!(", stringify!($ty), "::new(-", stringify!($float), "::NAN) < ",
                    stringify!($ty), "::new(", stringify!($float), "::NEG_INFINITY));",
                )]
        #[doc = concat!(
                    "assert!(", stringify!($ty), "::new(", stringify!($float), "::NAN) == ",
                    stringify!($ty), "::new(", stringify!($float), "::NAN));",
                )]
        /// ```
        impl Ord for $ty {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.total_cmp(&other.0)
            }
        }
    };
}

float_ty! { F32(f32) }
float_ty! { F64(f64) }

impl Number {
    /// Construct a new number.
    pub fn new(v: impl Into<Number>) -> Self {
        v.into()
    }

    /// Returns the [`f64`] representation of the [`Number`] regardless of
    /// whether the number is stored as a float or integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use ron_value::Number;
    /// let i = Number::new(5);
    /// let f = Number::new(2.0);
    /// assert_eq!(i.into_f64(), 5.0);
    /// assert_eq!(f.into_f64(), 2.0);
    /// ```
    #[must_use]
    pub fn into_f64(self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        match self {
            Self::U8(v) => f64::from(v),
            Self::U16(v) => f64::from(v),
            Self::U32(v) => f64::from(v),
            Self::U64(v) => v as f64,
            Self::U128(v) => v as f64,
            Self::USize(v) => v as f64,
            Self::I8(v) => f64::from(v),
            Self::I16(v) => f64::from(v),
            Self::I32(v) => f64::from(v),
            Self::I64(v) => v as f64,
            Self::I128(v) => v as f64,
            Self::ISize(v) => v as f64,
            Self::F32(v) => f64::from(v.get()),
            Self::F64(v) => v.get(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nan() {
        assert_eq!(F32(f32::NAN), F32(f32::NAN));
        assert_eq!(F32(-f32::NAN), F32(-f32::NAN));
        assert_ne!(F32(f32::NAN), F32(-f32::NAN));
    }

    #[test]
    fn test_nan_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn hash<T: Hash>(v: &T) -> u64 {
            let mut state = DefaultHasher::new();
            v.hash(&mut state);
            state.finish()
        }

        assert_eq!(hash(&F32(f32::NAN)), hash(&F32(f32::NAN)));
        assert_eq!(hash(&F32(-f32::NAN)), hash(&F32(-f32::NAN)));
        assert_ne!(hash(&F32(f32::NAN)), hash(&F32(-f32::NAN)));
    }

    #[test]
    fn test_partial_ord() {
        assert!(F32(f32::NAN) > F32(f32::INFINITY));
        assert!(F32(-f32::NAN) < F32(f32::NEG_INFINITY));
        assert!(F32(f32::NAN) == F32(f32::NAN));
    }
}
