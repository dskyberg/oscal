use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{PositiveIntegerDatatype, SchemaConstraint, StringDatatype};

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

impl SchemaConstraint for FrequencyCondition {
    fn constraint_title() -> &'static str {
        "Frequency Condition"
    }
    fn constraint_description() -> &'static str {
        "The task is intended to occur at the specified frequency."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing"
    }
}
