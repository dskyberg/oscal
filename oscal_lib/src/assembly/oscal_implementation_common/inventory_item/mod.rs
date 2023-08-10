pub use implemented_component::*;


pub mod implemented_component;

/// Inventory Item
/// A single managed inventory item within the system.
/// $id: #assembly_oscal-implementation-common_inventory-item
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
pub struct InventoryItem {
	/// Inventory Item Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this inventory item elsewhere in this or other OSCAL instances. The locally defined UUID of the inventory item can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub links: Option<Vec<Link>>,
	pub implemented_components: Option<Vec<ImplementedComponent>>,
	pub remarks: Option<Remarks>,
	pub responsible_parties: Option<Vec<ResponsibleParty>>,
	/// Inventory Item Description
	/// A summary of the inventory item stating its purpose within the system.
	pub description: String,
	pub props: Option<Vec<Property>>,
}
