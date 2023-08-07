pub use assessment_log_entry::*;


pub mod assessment_log_entry;

/// Assessment Log
/// A log of all assessment-related actions taken.
/// $id: #assembly_oscal-ar_result_assessment-log_assessment-log
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentLog {
	pub entries: Vec<AssessmentLogEntry>,
}
