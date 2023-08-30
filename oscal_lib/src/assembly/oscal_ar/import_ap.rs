/// Import Assessment Plan
/// Used by assessment-results to import information about the original plan for assessing the system.
/// $id: #assembly_oscal-ar_import-ap
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UriReferenceDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImportAp {
	/// Assessment Plan Reference
	/// A resolvable URL reference to the assessment plan governing the assessment activities.
	pub href: UriReferenceDatatype,
	pub remarks: Option<Remarks>,
}
