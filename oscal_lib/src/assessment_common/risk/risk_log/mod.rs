use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use self::risk_log_entry::RiskLogEntry;

pub mod risk_log_entry;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskLog {
    pub entries: Vec<RiskLogEntry>,
}

impl SchemaConstraint for RiskLog {
    fn constraint_title() -> &'static str {
        "Risk Log"
    }
    fn constraint_description() -> &'static str {
        "A log of all risk-related tasks taken."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:risk-log"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:risk-log"
    }
}
