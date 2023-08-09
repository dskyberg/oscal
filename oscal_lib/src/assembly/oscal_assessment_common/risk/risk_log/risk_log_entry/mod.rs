pub use risk_response_reference::*;


pub mod risk_response_reference;

/// Risk Log Entry
/// Identifies an individual risk response that occurred as part of managing an identified risk.
/// $id: #assembly_oscal-assessment-common_risk_risk-log_risk-log_risk-log-entry_risk-log-entry
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::LoggedBy;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::DateTimeWithTimezoneDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_assessment_common::RiskStatus;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct RiskLogEntry {
	pub props: Option<Vec<Property>>,
	/// Start
	/// Identifies the start date and time of the event.
	pub start: DateTimeWithTimezoneDatatype,
	pub status_change: Option<RiskStatus>,
	pub logged_by: Option<Vec<LoggedBy>>,
	/// Risk Log Entry Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this risk log entry elsewhere in this or other OSCAL instances. The locally defined UUID of the risk log entry can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// End
	/// Identifies the end date and time of the event. If the event is a point in time, the start and end will be the same date and time.
	pub end: Option<DateTimeWithTimezoneDatatype>,
	pub links: Option<Vec<Link>>,
	/// Title
	/// The title for this risk log entry.
	pub title: Option<String>,
	/// Risk Task Description
	/// A human-readable description of what was done regarding the risk.
	pub description: Option<String>,
	pub related_responses: Option<Vec<RiskResponseReference>>,
	pub remarks: Option<Remarks>,
}
