/// Control
/// A structured information object representing a security or privacy control. Each security or privacy control within the Catalog is defined by a distinct control instance.
/// $id: #assembly_oscal-catalog_control
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
pub struct Control {
	/// Control Identifier
	/// A human-oriented, locally unique identifier with instance scope that can be used to reference this control elsewhere in this and other OSCAL instances (e.g., profiles). This id should be assigned per-subject, which means it should be consistently used to identify the same control across revisions of the document.
	pub id: TokenDatatype,
	pub controls: Option<Vec<Control>>,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	pub params: Option<Vec<Parameter>>,
	pub parts: Option<Vec<Part>>,
	/// Control Title
	/// A name given to the control, which may be used by a tool for display and navigation.
	pub title: String,
	/// Control Class
	/// A textual label that provides a sub-type or characterization of the control.
	pub class: Option<TokenDatatype>,
}
