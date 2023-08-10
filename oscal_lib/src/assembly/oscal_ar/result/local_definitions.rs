/// Local Definitions
/// Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
/// $id: #assembly_oscal-ar_result_local-definitions_local-definitions
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentAssets;
use crate::assembly::oscal_assessment_common::Task;
use crate::assembly::oscal_implementation_common::InventoryItem;
use crate::assembly::oscal_implementation_common::SystemComponent;
use crate::assembly::oscal_implementation_common::SystemUser;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct LocalDefinitions {
	pub tasks: Option<Vec<Task>>,
	pub components: Option<Vec<SystemComponent>>,
	pub users: Option<Vec<SystemUser>>,
	pub inventory_items: Option<Vec<InventoryItem>>,
	pub assessment_assets: Option<AssessmentAssets>,
}
