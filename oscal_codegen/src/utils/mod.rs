use serde_json::{Map, Value};

use crate::{ParserError, Result};

pub use merge_ids::merge_ids;
pub use reserved_word::reserved_word;

mod merge_ids;
mod reserved_word;

pub fn str_from_value(value: &Value) -> Option<String> {
    value.as_str().map(|s| s.to_string())
}

pub fn try_str_from_map(item: &str, obj: &Map<String, Value>) -> Option<String> {
    if obj.contains_key(item) {
        return str_from_value(obj.get(item).unwrap());
    }
    None
}

pub fn try_number_from_map(item: &str, obj: &Map<String, Value>) -> Option<isize> {
    match try_str_from_map(item, obj) {
        Some(val) => val.parse::<isize>().ok(),
        None => None,
    }
}

pub fn str_from_map(item: &str, obj: &Map<String, Value>) -> Result<String> {
    match try_str_from_map(item, obj) {
        Some(s) => Ok(s),
        None => Err(ParserError::MissingField(item.to_string()).into()),
    }
}
