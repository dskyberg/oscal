use serde_json::{Map, Value};

use crate::{ParserError, Result};

pub use merge_ids::merge_ids;
pub use reserved_word::reserved_word;

mod merge_ids;
mod reserved_word;

pub fn str_from_value(value: &Value) -> Result<&str> {
    match value.is_string() {
        true => Ok(value.as_str().unwrap()),
        false => Err(ParserError::StringExpected.into()),
    }
}

pub fn number_from_value(value: &Value) -> Result<i64> {
    match value.is_i64() {
        true => Ok(value.as_i64().unwrap()),
        false => Err(ParserError::NumberExpected.into()),
    }
}

pub fn try_str_from_map<'a>(key: &str, obj: &'a Map<String, Value>) -> Result<Option<&'a str>> {
    match obj.get(key) {
        Some(val) => Ok(Some(str_from_value(val)?)),
        None => Ok(None),
    }
}

pub fn try_number_from_map(key: &str, obj: &Map<String, Value>) -> Result<Option<i64>> {
    match obj.get(key) {
        Some(val) => Ok(Some(number_from_value(val)?)),
        None => Ok(None),
    }
}

pub fn str_from_map<'a>(key: &str, obj: &'a Map<String, Value>) -> Result<&'a str> {
    match try_str_from_map(key, obj)? {
        Some(s) => Ok(s),
        None => Err(ParserError::MissingField(key.to_string()).into()),
    }
}

#[allow(dead_code)]
pub fn try_obj_from_map<'a>(
    key: &str,
    obj: &'a Map<String, Value>,
) -> Result<Option<&'a Map<String, Value>>> {
    match obj.get(key) {
        Some(val) => Ok(Some(val.as_object().ok_or(ParserError::ObjectExpected)?)),
        None => Ok(None),
    }
}

#[allow(dead_code)]
pub fn obj_from_map<'a>(key: &str, obj: &'a Map<String, Value>) -> Result<&'a Map<String, Value>> {
    match try_obj_from_map(key, obj)? {
        Some(a) => Ok(a),
        None => Err(ParserError::MissingField(key.to_string()).into()),
    }
}

pub fn try_array_from_map<'a>(
    key: &str,
    obj: &'a Map<String, Value>,
) -> Result<Option<&'a Vec<Value>>> {
    match obj.get(key) {
        Some(val) => Ok(Some(val.as_array().ok_or(ParserError::ArrayExpected)?)),
        None => Ok(None),
    }
}
pub fn array_from_map<'a>(key: &str, obj: &'a Map<String, Value>) -> Result<&'a Vec<Value>> {
    match try_array_from_map(key, obj)? {
        Some(a) => Ok(a),
        None => Err(ParserError::MissingField(key.to_string()).into()),
    }
}
