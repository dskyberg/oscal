use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaConstraint, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ParameterValue(StringDatatype);

impl Deref for ParameterValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for ParameterValue {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(StringDatatype::try_from(value)?))
    }
}

impl SchemaConstraint for ParameterValue {
    fn constraint_title() -> &'static str {
        "Parameter Value"
    }
    fn constraint_description() -> &'static str {
        "A parameter value or set of values."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-catalog-common_parameter-value"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:parameter-value"
    }
}
