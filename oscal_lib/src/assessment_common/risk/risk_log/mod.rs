use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

use self::risk_log_entry::RiskLogEntry;

pub mod risk_log_entry;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskLog {
    pub entries: Vec<RiskLogEntry>,
}

impl SchemaElement for RiskLog {
    fn schema_title() -> &'static str {
        "Risk Log"
    }
    fn schema_description() -> &'static str {
        "A log of all risk-related tasks taken."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:risk-log"
    }
}
