/// Local Definitions
/// Allows components, and inventory-items to be defined within the POA&M for circumstances where no OSCAL-based SSP exists, or is not delivered with the POA&M.
/// $id: #assembly_oscal-poam_local-definitions
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::InventoryItem;
use crate::assembly::oscal_implementation_common::SystemComponent;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct LocalDefinitions {
	pub inventory_items: Option<Vec<InventoryItem>>,
	pub remarks: Option<Remarks>,
	pub components: Option<Vec<SystemComponent>>,
}
