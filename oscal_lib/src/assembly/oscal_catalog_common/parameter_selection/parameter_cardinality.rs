/// Parameter Cardinality
/// Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.
/// $id: #assembly_oscal-catalog-common_parameter-selection_parameter-cardinality_parameter-cardinality
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ParameterCardinality {
	// orig: one
	One,
	// orig: one-or-more
	OneOrMore,
}
