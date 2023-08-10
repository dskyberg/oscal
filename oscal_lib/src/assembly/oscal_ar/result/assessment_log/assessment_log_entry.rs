/// Assessment Log Entry
/// Identifies the result of an action and/or task that occurred as part of executing an assessment plan or an assessment event that occurred in producing the assessment results.
/// $id: #assembly_oscal-ar_result_assessment-log_assessment-log_assessment-log-entry_assessment-log-entry
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::LoggedBy;
use crate::assembly::oscal_assessment_common::RelatedTask;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::DateTimeWithTimezoneDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentLogEntry {
	/// Action Description
	/// A human-readable description of this event.
	pub description: Option<String>,
	/// Start
	/// Identifies the start date and time of an event.
	pub start: DateTimeWithTimezoneDatatype,
	pub links: Option<Vec<Link>>,
	/// Assessment Log Entry Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference an assessment event in this or other OSCAL instances. The locally defined UUID of the assessment log entry can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub logged_by: Option<Vec<LoggedBy>>,
	pub props: Option<Vec<Property>>,
	pub related_tasks: Option<Vec<RelatedTask>>,
	pub remarks: Option<Remarks>,
	/// End
	/// Identifies the end date and time of an event. If the event is a point in time, the start and end will be the same date and time.
	pub end: Option<DateTimeWithTimezoneDatatype>,
	/// Action Title
	/// The title for this event.
	pub title: Option<String>,
}
