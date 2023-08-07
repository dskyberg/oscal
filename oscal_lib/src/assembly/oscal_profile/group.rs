/// Control group
/// A group of (selected) controls or of groups of controls
/// $id: #assembly_oscal-profile_group
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog_common::Parameter;
use crate::assembly::oscal_catalog_common::Part;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_profile::InsertControls;
use crate::definitions::TokenDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Group {
	pub insert_controls: Option<Vec<InsertControls>>,
	pub links: Option<Vec<Link>>,
	pub groups: Option<Vec<Group>>,
	pub props: Option<Vec<Property>>,
	pub params: Option<Vec<Parameter>>,
	/// Group Identifier
	/// A human-oriented, locally unique identifier with cross-instance scope that can be used to reference this defined group elsewhere in this or other OSCAL instances. When referenced from another OSCAL instance, this identifier must be referenced in the context of the containing resource (e.g., import-profile). This id should be assigned per-subject, which means it should be consistently used to identify the same group across revisions of the document.
	pub id: Option<TokenDatatype>,
	/// Group Class
	/// A textual label that provides a sub-type or characterization of the group.
	pub class: Option<TokenDatatype>,
	/// Group Title
	/// A name given to the group, which may be used by a tool for display and navigation.
	pub title: String,
	pub parts: Option<Vec<Part>>,
}
