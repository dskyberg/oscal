/// Role
/// Defines a function assumed or expected to be assumed by a party in a specific situation.
/// $id: #assembly_oscal-metadata_role
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::StringDatatype;
use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Role {
	/// Role Title
	/// A name given to the role, which may be used by a tool for display and navigation.
	pub title: String,
	pub props: Option<Vec<Property>>,
	/// Role Identifier
	/// A human-oriented, locally unique identifier with cross-instance scope that can be used to reference this defined role elsewhere in this or other OSCAL instances. When referenced from another OSCAL instance, the locally defined ID of the Role from the imported OSCAL instance must be referenced in the context of the containing resource (e.g., import, import-component-definition, import-profile, import-ssp or import-ap). This ID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub id: TokenDatatype,
	pub links: Option<Vec<Link>>,
	/// Role Description
	/// A summary of the role's purpose and associated responsibilities.
	pub description: Option<String>,
	pub remarks: Option<Remarks>,
	/// Role Short Name
	/// A short common name, abbreviation, or acronym for the role.
	pub short_name: Option<StringDatatype>,
}
