/// Import resource
/// The import designates a catalog or profile to be included (referenced and potentially modified) by this profile. The import also identifies which controls to select using the include-all, include-controls, and exclude-controls directives.
/// $id: #assembly_oscal-profile_import
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog_common::IncludeAll;
use crate::assembly::oscal_profile::SelectControlById;
use crate::definitions::UriReferenceDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Import {
	pub exclude_controls: Option<Vec<SelectControlById>>,
	pub include_all: Option<IncludeAll>,
	pub include_controls: Option<Vec<SelectControlById>>,
	/// Catalog or Profile Reference
	/// A resolvable URL reference to the base catalog or profile that this profile is tailoring.
	pub href: UriReferenceDatatype,
}
