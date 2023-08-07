pub use parameter_cardinality::*;


pub mod parameter_cardinality;

/// Selection
/// Presenting a choice among alternatives
/// $id: #assembly_oscal-catalog-common_parameter-selection
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ParameterSelection {
	/// Parameter Cardinality
	/// Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.
	pub how_many: Option<ParameterCardinality>,
	pub choice: Option<Vec<String>>,
}
