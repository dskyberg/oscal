/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/profile.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
}

impl SchemaConstraint for Profile {
    fn constraint_title() -> &'static str {
        "Profile"
    }
    fn constraint_description() -> &'static str {
        r#"Each OSCAL profile is defined by a Profile element"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_profile"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:profile"
    }
}
