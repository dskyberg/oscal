/// Related Observation
/// Relates the poam-item to a set of referenced observations that were used to determine the finding.
/// $id: #assembly_oscal-poam_poam-item_related-observation_related-observation
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct RelatedObservation {
	/// Observation Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to an observation defined in the list of observations.
	pub observation_uuid: UuidDatatype,
}
