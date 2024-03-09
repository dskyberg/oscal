/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/adjustment_justification.rs
use serde::{Deserialize, Serialize};

use crate::SchemaElement;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AdjustmentJustification(String);

impl std::ops::Deref for AdjustmentJustification {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for AdjustmentJustification {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl SchemaElement for AdjustmentJustification {
    fn schema_title() -> &'static str {
        "Adjustment Justification"
    }
    fn schema_description() -> &'static str {
        r#"If the selected security level is different from the base security level, this contains the justification for the change."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-ssp_adjustment-justification")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:adjustment-justification"
    }
}
