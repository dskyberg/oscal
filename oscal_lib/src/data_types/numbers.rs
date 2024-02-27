use serde::{Deserialize, Serialize};
use std::ops::Deref;

use super::{DecimalType, NumberType};
use crate::SchemaConstraint;

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

impl SchemaConstraint for DecimalDatatype {
    fn constraint_title() -> &'static str {
        "Decimal"
    }
    fn constraint_description() -> &'static str {
        "A real number expressed using a whole and optional fractional part separated by a period."
    }
    fn constraint_id() -> &'static str {
        "DecimalDatatype"
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

impl NumberType for IntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(0)
    }
}

impl SchemaConstraint for IntegerDatatype {
    fn constraint_title() -> &'static str {
        "Integer"
    }
    fn constraint_description() -> &'static str {
        "In XML Schema this is represented as a restriction on the built-in type integer as follows:"
    }
    fn constraint_id() -> &'static str {
        "IntegerDatatype"
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

impl SchemaConstraint for NonNegativeIntegerDatatype {
    fn constraint_title() -> &'static str {
        "NonNegative Integer"
    }
    fn constraint_description() -> &'static str {
        "An integer value that is equal to or greater than 0."
    }
    fn constraint_id() -> &'static str {
        "NonNegativeIntegerDatatype"
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

impl SchemaConstraint for PositiveIntegerDatatype {
    fn constraint_title() -> &'static str {
        "Positive Integer"
    }
    fn constraint_description() -> &'static str {
        "An integer value that is greater than 0."
    }
    fn constraint_id() -> &'static str {
        "PositiveIntegerDatatype"
    }
    fn schema_path() -> &'static str {
        "positive-integer"
    }
}
