use crate::RustSchemaTrait;

macro_rules! wrapper_impl {
    ($($desc:tt)+) => {
        forward_impl!(($($desc)+ where T: RustSchemaTrait) => T);
    };
}

wrapper_impl!(<T> RustSchemaTrait for Box<T>);
