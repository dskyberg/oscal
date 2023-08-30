pub use assessment_platform::*;


pub mod assessment_platform;

/// Assessment Assets
/// Identifies the assets used to perform this assessment, such as the assessment team, scanning tools, and assumptions.
/// $id: #assembly_oscal-assessment-common_assessment-assets
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::SystemComponent;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentAssets {
	pub assessment_platforms: Vec<AssessmentPlatform>,
	pub components: Option<Vec<SystemComponent>>,
}
