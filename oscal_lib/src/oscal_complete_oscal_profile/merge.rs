/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/merge.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Merge {
}

impl SchemaConstraint for Merge {
    fn constraint_title() -> &'static str {
        "Merge controls"
    }
    fn constraint_description() -> &'static str {
        r#"A Merge element provides structuring directives that drive how controls are organized after resolution."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_merge"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:merge"
    }
}
