/// Implemented Component
/// The set of components that are implemented in a given system inventory item.
/// $id: #assembly_oscal-implementation-common_inventory-item_implemented-component_implemented-component
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleParty;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImplementedComponent {
	pub responsible_parties: Option<Vec<ResponsibleParty>>,
	pub links: Option<Vec<Link>>,
	/// Component Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a component that is implemented as part of an inventory item.
	pub component_uuid: UuidDatatype,
	pub remarks: Option<Remarks>,
	pub props: Option<Vec<Property>>,
}
