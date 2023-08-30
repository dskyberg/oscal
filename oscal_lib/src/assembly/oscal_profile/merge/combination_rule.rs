/// Combination rule
/// A Combine element defines how to combine multiple (competing) versions of the same control.
/// $id: #assembly_oscal-profile_merge_combination-rule_combination-rule
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct CombinationRule {
	/// Combination method
	/// How clashing controls should be handled
	pub method: Option<StringDatatype>,
}
