use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TransparentType};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", transparent)]
pub struct ParameterValue {
    pub inner: StringDatatype,
}
impl TransparentType<String> for ParameterValue {
    fn inner(&self) -> &String {
        self.inner.inner()
    }
}
impl From<&str> for ParameterValue {
    fn from(value: &str) -> Self {
        Self {
            inner: StringDatatype::from(value),
        }
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
        "value"
    }
}
