use macros::{
    cached_fn, constant_string, hash_mapify, log_duration, DeriveCustomModel, IntoStringHashMap,
};
use std::str::from_utf8;

#[derive(IntoStringHashMap)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
}

#[derive(DeriveCustomModel)]
#[custom_model(model(
    name = "UserName",
    fields(first_name, last_name),
    extra_derives(IntoStringHashMap)
))]
#[custom_model(model(name = "UserInfo", fields(username, age), extra_derives(Debug)))]
pub struct User2 {
    username: String,
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    test_cache("John".to_string(), "Appleseed".to_string());
    test_cache("John".to_string(), "Appleseed".to_string());
    test_cache("John".to_string(), "Doe".to_string());
}

#[log_duration]
#[must_use]
fn function_to_benchmark() -> u16 {
    let mut counter = 0;
    for _ in 0..u16::MAX {
        counter += 1;
    }

    counter
}

constant_string!("SOME_CONSTANT_STRING_VALUE");

#[cached_fn(keygen = "format!(\"{first_name} {last_name}\")")]
fn test_cache(first_name: String, last_name: String) -> String {
    format!("{first_name} {last_name}")
}

fn test_hashmap() {
    let some_variable = "Some variable value";

    let hash_map = hash_mapify!(
        &str,
        "first_key" = "first_value",
        "second_variable" = some_variable,
        some_key = "value for variable key",
    );

    let number_hash_map =
        hash_mapify!(usize, "first_key" = 1, "second_variable" = 2, some_key = 3,);

    dbg!(hash_map);
    dbg!(number_hash_map);
}
