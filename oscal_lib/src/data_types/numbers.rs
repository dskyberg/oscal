use serde::{Deserialize, Serialize};
use std::ops::Deref;

use super::{DecimalType, NumberType};
use crate::SchemaElement;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DecimalDatatype(f64);

impl Deref for DecimalDatatype {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for DecimalDatatype {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl DecimalType for DecimalDatatype {}

impl SchemaElement for DecimalDatatype {
    fn schema_title() -> &'static str {
        "Decimal"
    }
    fn schema_description() -> &'static str {
        "A real number expressed using a whole and optional fractional part separated by a period."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "decimal"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IntegerDatatype(i64);

impl Deref for IntegerDatatype {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i64> for IntegerDatatype {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl NumberType for IntegerDatatype {}

impl SchemaElement for IntegerDatatype {
    fn schema_title() -> &'static str {
        "Integer"
    }
    fn schema_description() -> &'static str {
        "In XML Schema this is represented as a restriction on the built-in type integer as follows:"
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "integer"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NonNegativeIntegerDatatype(u64);

impl Deref for NonNegativeIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for NonNegativeIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl NumberType for NonNegativeIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(0)
    }
}

impl SchemaElement for NonNegativeIntegerDatatype {
    fn schema_title() -> &'static str {
        "NonNegative Integer"
    }
    fn schema_description() -> &'static str {
        "An integer value that is equal to or greater than 0."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "non-negative-integer"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PositiveIntegerDatatype(u64);

impl Deref for PositiveIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for PositiveIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl NumberType for PositiveIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(1)
    }
}

impl SchemaElement for PositiveIntegerDatatype {
    fn schema_title() -> &'static str {
        "Positive Integer"
    }
    fn schema_description() -> &'static str {
        "An integer value that is greater than 0."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "positive-integer"
    }
}
