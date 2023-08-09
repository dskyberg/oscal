pub use risk_log_entry::*;


pub mod risk_log_entry;

/// Risk Log
/// A log of all risk-related tasks taken.
/// $id: #assembly_oscal-assessment-common_risk_risk-log_risk-log
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct RiskLog {
	pub entries: Vec<RiskLogEntry>,
}
