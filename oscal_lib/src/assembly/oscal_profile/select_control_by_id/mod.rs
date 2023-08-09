pub use match_controls_by_pattern::*;
pub use include_contained_controls_with_control::*;


pub mod match_controls_by_pattern;
pub mod include_contained_controls_with_control;

/// Call
/// Call a control by its ID
/// $id: #assembly_oscal-profile_select-control-by-id
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::TokenDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SelectControlById {
	pub matching: Option<Vec<MatchControlsByPattern>>,
	/// Include contained controls with control
	/// When a control is included, whether its child (dependent) controls are also included.
	pub with_child_controls: Option<IncludeContainedControlsWithControl>,
	pub with_ids: Option<Vec<TokenDatatype>>,
}
