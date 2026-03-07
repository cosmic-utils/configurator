use std::path::Path;

use snapbox::data::DataFormat;

#[macro_export]
macro_rules! test_name {
    () => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let test_fn_name = type_name_of_val(f)
            .trim_end_matches("::f")
            .split("::")
            .last()
            .unwrap();
        format!("{}~{}", core::file!(), test_fn_name)
    }};
}

#[macro_export]
macro_rules! test {
    ($type:ty) => {{
        let schema = schema_for::<$type>();
        let json = json::to_string_pretty(&schema).unwrap();

        $crate::common::assert_snapshot(&$crate::test_name!(), &json)
    }};
}

pub fn assert_snapshot(name: &str, value: &str) {
    let file = format!("rust_schema2/tests/snapshots/{}.json", name);

    let data = snapbox::Data::read_from(Path::new(&file), Some(DataFormat::Json)).raw();

    snapbox::assert_data_eq!(value, data);
}
