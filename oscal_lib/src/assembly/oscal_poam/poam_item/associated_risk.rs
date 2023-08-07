/// Associated Risk
/// Relates the finding to a set of referenced risks that were used to determine the finding.
/// $id: #assembly_oscal-poam_poam-item_associated-risk_associated-risk
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssociatedRisk {
	/// Risk Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a risk defined in the list of risks.
	pub risk_uuid: UuidDatatype,
}
