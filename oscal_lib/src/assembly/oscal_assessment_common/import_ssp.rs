/// Import System Security Plan
/// Used by the assessment plan and POA&M to import information about the system.
/// $id: #assembly_oscal-assessment-common_import-ssp
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UriReferenceDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImportSsp {
	pub remarks: Option<Remarks>,
	/// System Security Plan Reference
	/// A resolvable URL reference to the system security plan for the system being assessed.
	pub href: UriReferenceDatatype,
}
