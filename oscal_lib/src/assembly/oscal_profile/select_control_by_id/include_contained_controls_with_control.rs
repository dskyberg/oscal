/// Include contained controls with control
/// When a control is included, whether its child (dependent) controls are also included.
/// $id: #assembly_oscal-profile_select-control-by-id_include-contained-controls-with-control_include-contained-controls-with-control
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum IncludeContainedControlsWithControl {
	// orig: yes
	Yes,
	// orig: no
	No,
}
