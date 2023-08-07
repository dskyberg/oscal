/// Match Controls by Pattern
/// Select controls by (regular expression) match on ID
/// $id: #assembly_oscal-profile_select-control-by-id_match-controls-by-pattern_match-controls-by-pattern
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct MatchControlsByPattern {
	/// Pattern
	/// A glob expression matching the IDs of one or more controls to be selected.
	pub pattern: Option<StringDatatype>,
}
