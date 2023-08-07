/// Local Definitions
/// Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
/// $id: #assembly_oscal-ar_assessment-results_local-definitions_local-definitions
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::Activity;
use crate::assembly::oscal_assessment_common::LocalObjective;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct LocalDefinitions {
	pub remarks: Option<Remarks>,
	pub activities: Option<Vec<Activity>>,
	pub objectives_and_methods: Option<Vec<LocalObjective>>,
}
