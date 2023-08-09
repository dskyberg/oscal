/// Custom grouping
/// A Custom element frames a structure for embedding represented controls in resolution.
/// $id: #assembly_oscal-profile_merge_custom-grouping_custom-grouping
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_profile::Group;
use crate::assembly::oscal_profile::InsertControls;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct CustomGrouping {
	pub insert_controls: Option<Vec<InsertControls>>,
	pub groups: Option<Vec<Group>>,
}
