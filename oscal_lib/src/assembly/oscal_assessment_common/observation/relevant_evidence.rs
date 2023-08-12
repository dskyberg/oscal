/// Relevant Evidence
/// Links this observation to relevant evidence.
/// $id: #assembly_oscal-assessment-common_observation_relevant-evidence_relevant-evidence
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UriReferenceDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct RelevantEvidence {
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Relevant Evidence Description
	/// A human-readable description of this evidence.
	pub description: String,
	/// Relevant Evidence Reference
	/// A resolvable URL reference to relevant evidence.
	pub href: Option<UriReferenceDatatype>,
	pub remarks: Option<Remarks>,
}
