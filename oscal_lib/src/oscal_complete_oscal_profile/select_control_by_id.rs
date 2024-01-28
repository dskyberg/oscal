/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/select_control_by_id.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectControlById {
}

impl SchemaConstraint for SelectControlById {
    fn constraint_title() -> &'static str {
        "Call"
    }
    fn constraint_description() -> &'static str {
        r#"Call a control by its ID"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_select-control-by-id"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:select-control-by-id"
    }
}
