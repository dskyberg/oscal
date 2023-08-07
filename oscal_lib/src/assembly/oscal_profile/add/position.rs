/// Position
/// Where to add the new content with respect to the targeted element (beside it or inside it)
/// $id: #assembly_oscal-profile_add_position_position
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum Position {
	// orig: before
	Before,
	// orig: after
	After,
	// orig: starting
	Starting,
	// orig: ending
	Ending,
}
