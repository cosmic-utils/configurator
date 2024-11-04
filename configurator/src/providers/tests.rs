use std::{collections::HashMap, fs, marker::PhantomData, path::Path, sync::LazyLock};

use configurator_utils::ConfigFormat;
use figment::{
    value::{Tag, Value},
    Profile, Provider,
};
use serde::Serialize;
use serial_test::serial;

use crate::test_common::*;

use pretty_assertions::assert_eq;

/// 1. write the value
/// 2. read the value and assert equal
fn write_and_read<P: AsRef<Path>>(path: P, format: &ConfigFormat, initial_value: &Value) {
    let _ = fs::remove_dir_all(path.as_ref());

    super::write(path.as_ref(), format, initial_value).unwrap();

    let value = super::read_from_format(path.as_ref(), format);

    let value = value.data().unwrap().remove(&Profile::Default).unwrap();

    let value = Value::Dict(Tag::Default, value);

    assert_eq!(initial_value, &value);
}

fn write_and_read_common<S: Default + Serialize>(format: &ConfigFormat) {
    write_and_read(
        "tests/cosmic_ron/config1",
        format,
        &Value::serialize(S::default()).unwrap(),
    );
}

#[test]
#[serial]
fn test_bool_ron() {
    write_and_read_common::<TestBool>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_string_ron() {
    write_and_read_common::<TestString>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_number_ron() {
    write_and_read_common::<TestNumber>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_float_ron() {
    write_and_read_common::<TestFloat>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_enum_simple_ron() {
    write_and_read_common::<TestEnumSimple>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_enum_complex_ron() {
    write_and_read_common::<TestEnumComplex>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_option_ron() {
    write_and_read_common::<TestOption>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_option_complex_ron() {
    write_and_read_common::<TestOptionComplex>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_tuple_ron() {
    write_and_read_common::<TestTuple>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_vec_ron() {
    write_and_read_common::<TestVec>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_hash_map_ron() {
    write_and_read_common::<TestHashMap>(&ConfigFormat::CosmicRon);
}

#[test]
#[serial]
fn test_very_complex_ron() {
    write_and_read_common::<TestVeryComplex>(&ConfigFormat::CosmicRon);
}
