/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/modify.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

use super::alter::Alter;

use self::parameter_setting::ParameterSetting;

pub mod parameter_setting;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Modify {
    pub set_parameters: Option<Vec<ParameterSetting>>,
    pub alters: Option<Vec<Alter>>,
}

impl SchemaElement for Modify {
    fn schema_title() -> &'static str {
        "Modify controls"
    }
    fn schema_description() -> &'static str {
        r#"Set parameters or amend controls in resolution"#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_modify")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:modify"
    }
}
