/// Addition
/// Specifies contents to be added into controls, in resolution
/// $id: #assembly_oscal-profile_add
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog_common::Parameter;
use crate::assembly::oscal_catalog_common::Part;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Add {
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	pub parts: Option<Vec<Part>>,
	/// Position
	/// Where to add the new content with respect to the targeted element (beside it or inside it)
	pub position: Option<TokenDatatype>,
	/// Title Change
	/// A name given to the control, which may be used by a tool for display and navigation.
	pub title: Option<String>,
	/// Reference by ID
	/// Target location of the addition.
	pub by_id: Option<TokenDatatype>,
	pub params: Option<Vec<Parameter>>,
}
