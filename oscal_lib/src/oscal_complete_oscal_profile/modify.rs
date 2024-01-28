/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/modify.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Modify {
}

impl SchemaConstraint for Modify {
    fn constraint_title() -> &'static str {
        "Modify controls"
    }
    fn constraint_description() -> &'static str {
        r#"Set parameters or amend controls in resolution"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_modify"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:modify"
    }
}
