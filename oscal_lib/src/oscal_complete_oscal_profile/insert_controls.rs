/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/insert_controls.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertControls {
}

impl SchemaConstraint for InsertControls {
    fn constraint_title() -> &'static str {
        "Select controls"
    }
    fn constraint_description() -> &'static str {
        r#"Specifies which controls to use in the containing context."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_insert-controls"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:insert-controls"
    }
}
