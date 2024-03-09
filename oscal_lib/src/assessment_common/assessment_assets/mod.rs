/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_assets.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{implementation_common::system_component::SystemComponent, SchemaElement};

use self::assessment_platform::AssessmentPlatform;

pub mod assessment_platform;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentAssets {
    pub components: Option<Vec<SystemComponent>>,
    pub assessment_platforms: Vec<AssessmentPlatform>,
}

impl SchemaElement for AssessmentAssets {
    fn schema_title() -> &'static str {
        "Assessment Assets"
    }
    fn schema_description() -> &'static str {
        r#"Identifies the assets used to perform this assessment, such as the assessment team, scanning tools, and assumptions."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-assets"
    }
}
