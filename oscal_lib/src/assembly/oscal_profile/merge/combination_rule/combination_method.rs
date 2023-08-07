/// Combination method
/// How clashing controls should be handled
/// $id: #assembly_oscal-profile_merge_combination-rule_combination-rule_combination-method_combination-method
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum CombinationMethod {
	// orig: use-first
	UseFirst,
	// orig: merge
	Merge,
	// orig: keep
	Keep,
}
