pub use order::*;


pub mod order;

/// Select controls
/// Specifies which controls to use in the containing context.
/// $id: #assembly_oscal-profile_insert-controls
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog_common::IncludeAll;
use crate::assembly::oscal_profile::SelectControlById;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct InsertControls {
	pub include_controls: Option<Vec<SelectControlById>>,
	/// Order
	/// A designation of how a selection of controls in a profile is to be ordered.
	pub order: Option<Order>,
	pub exclude_controls: Option<Vec<SelectControlById>>,
	pub include_all: Option<IncludeAll>,
}
