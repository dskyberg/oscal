use serde::{Deserialize, Serialize};

use crate::{Error, SchemaElement, TokenDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RiskStatus(TokenDatatype);

impl SchemaElement for RiskStatus {
    fn schema_title() -> &'static str {
        "Risk Status"
    }

    fn schema_description() -> &'static str {
        "Describes the status of the associated risk."
    }

    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-assessment-common_risk-status")
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk-status"
    }
}

impl TryFrom<&str> for RiskStatus {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(TokenDatatype::try_from(value)?))
    }
}
