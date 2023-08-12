pub use related_observation::*;
pub use associated_risk::*;


pub mod related_observation;
pub mod associated_risk;

/// Finding
/// Describes an individual finding.
/// $id: #assembly_oscal-ar_finding
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::FindingTarget;
use crate::assembly::oscal_assessment_common::Origin;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Finding {
	pub related_observations: Option<Vec<RelatedObservation>>,
	/// Implementation Statement UUID
	/// A machine-oriented identifier reference to the implementation statement in the SSP to which this finding is related.
	pub implementation_statement_uuid: Option<UuidDatatype>,
	pub remarks: Option<Remarks>,
	pub related_risks: Option<Vec<AssociatedRisk>>,
	pub origins: Option<Vec<Origin>>,
	pub props: Option<Vec<Property>>,
	/// Finding Description
	/// A human-readable description of this finding.
	pub description: String,
	pub target: FindingTarget,
	pub links: Option<Vec<Link>>,
	/// Finding Title
	/// The title for this finding.
	pub title: String,
	/// Finding Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this finding in this or other OSCAL instances. The locally defined UUID of the finding can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
}
