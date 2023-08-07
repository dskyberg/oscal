pub use parameter_setting::*;


pub mod parameter_setting;

/// Modify controls
/// Set parameters or amend controls in resolution
/// $id: #assembly_oscal-profile_modify
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_profile::Alter;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Modify {
	pub set_parameters: Option<Vec<ParameterSetting>>,
	pub alters: Option<Vec<Alter>>,
}
