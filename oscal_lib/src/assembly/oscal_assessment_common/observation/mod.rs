pub use observation_method::*;
pub use relevant_evidence::*;
pub use observation_type::*;


pub mod observation_method;
pub mod relevant_evidence;
pub mod observation_type;

/// Observation
/// Describes an individual observation.
/// $id: #assembly_oscal-assessment-common_observation
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::Origin;
use crate::assembly::oscal_assessment_common::SubjectReference;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::DateTimeWithTimezoneDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Observation {
	pub subjects: Option<Vec<SubjectReference>>,
	pub types: Option<Vec<ObservationType>>,
	/// Observation Description
	/// A human-readable description of this assessment observation.
	pub description: String,
	/// Observation Title
	/// The title for this observation.
	pub title: Option<String>,
	pub links: Option<Vec<Link>>,
	pub relevant_evidence: Option<Vec<RelevantEvidence>>,
	/// Collected Field
	/// Date/time stamp identifying when the finding information was collected.
	pub collected: DateTimeWithTimezoneDatatype,
	pub remarks: Option<Remarks>,
	/// Observation Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this observation elsewhere in this or other OSCAL instances. The locally defined UUID of the observation can be used to reference the data item locally or globally (e.g., in an imorted OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Expires Field
	/// Date/time identifying when the finding information is out-of-date and no longer valid. Typically used with continuous assessment scenarios.
	pub expires: Option<DateTimeWithTimezoneDatatype>,
	pub origins: Option<Vec<Origin>>,
	pub methods: Vec<ObservationMethod>,
	pub props: Option<Vec<Property>>,
}
