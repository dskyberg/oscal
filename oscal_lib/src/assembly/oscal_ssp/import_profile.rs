/// Import Profile
/// Used to import the OSCAL profile representing the system's control baseline.
/// $id: #assembly_oscal-ssp_import-profile
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UriReferenceDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImportProfile {
	/// Profile Reference
	/// A resolvable URL reference to the profile or catalog to use as the system's control baseline.
	pub href: UriReferenceDatatype,
	pub remarks: Option<Remarks>,
}
