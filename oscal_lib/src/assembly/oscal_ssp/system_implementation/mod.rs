pub use leveraged_authorization::*;


pub mod leveraged_authorization;

/// System Implementation
/// Provides information as to how the system is implemented.
/// $id: #assembly_oscal-ssp_system-implementation
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::InventoryItem;
use crate::assembly::oscal_implementation_common::SystemComponent;
use crate::assembly::oscal_implementation_common::SystemUser;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SystemImplementation {
	pub components: Vec<SystemComponent>,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
	pub users: Vec<SystemUser>,
	pub inventory_items: Option<Vec<InventoryItem>>,
	pub links: Option<Vec<Link>>,
	pub leveraged_authorizations: Option<Vec<LeveragedAuthorization>>,
}
