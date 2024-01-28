use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{SchemaConstraint, TokenDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RiskStatus(TokenDatatype);

impl SchemaConstraint for RiskStatus {
    fn constraint_title() -> &'static str {
        "Risk Status"
    }

    fn constraint_description() -> &'static str {
        "Describes the status of the associated risk."
    }

    fn constraint_id() -> &'static str {
        "#field_oscal-assessment-common_risk-status"
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk-status"
    }
}

impl Deref for RiskStatus {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<&str> for RiskStatus {
    fn from(value: &str) -> Self {
        Self(TokenDatatype::from(value))
    }
}
