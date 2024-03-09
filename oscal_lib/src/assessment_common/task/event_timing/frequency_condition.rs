use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{PositiveIntegerDatatype, SchemaElement, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FrequencyCondition {
    pub period: PositiveIntegerDatatype,
    /// "enum": [
    ///    "seconds",
    ///    "minutes",
    ///    "hours",
    ///    "days",
    ///    "months",
    ///   "years"
    /// ]
    pub unit: StringDatatype,
}

impl SchemaElement for FrequencyCondition {
    fn schema_title() -> &'static str {
        "Frequency Condition"
    }
    fn schema_description() -> &'static str {
        "The task is intended to occur at the specified frequency."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing"
    }
}
