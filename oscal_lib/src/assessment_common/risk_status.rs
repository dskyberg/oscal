use serde::{Deserialize, Serialize};

use crate::{Error, SchemaConstraint, TokenDatatype};

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

impl TryFrom<&str> for RiskStatus {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(TokenDatatype::try_from(value)?))
    }
}
