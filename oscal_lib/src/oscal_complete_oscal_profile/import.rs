/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/import.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Import {
}

impl SchemaConstraint for Import {
    fn constraint_title() -> &'static str {
        "Import resource"
    }
    fn constraint_description() -> &'static str {
        r#"The import designates a catalog or profile to be included (referenced and potentially modified) by this profile. The import also identifies which controls to select using the include-all, include-controls, and exclude-controls directives."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_import"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:import"
    }
}
