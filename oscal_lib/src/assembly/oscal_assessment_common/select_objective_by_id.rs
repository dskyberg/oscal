/// Select Objective
/// Used to select a control objective for inclusion/exclusion based on the control objective's identifier.
/// $id: #assembly_oscal-assessment-common_select-objective-by-id
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::TokenDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SelectObjectiveById {
	/// Objective ID
	/// Points to an assessment objective.
	pub objective_id: TokenDatatype,
}
