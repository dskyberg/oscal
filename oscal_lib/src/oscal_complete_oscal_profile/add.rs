/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/add.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Add {
}

impl SchemaConstraint for Add {
    fn constraint_title() -> &'static str {
        "Addition"
    }
    fn constraint_description() -> &'static str {
        r#"Specifies contents to be added into controls, in resolution"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_add"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:add"
    }
}
