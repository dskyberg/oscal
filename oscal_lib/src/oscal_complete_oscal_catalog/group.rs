/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog/group.rs
/// pub use oscal_complete_oscal_catalog::*;
///
/// pub mod oscal_complete_oscal_catalog;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
}

impl SchemaConstraint for Group {
    fn constraint_title() -> &'static str {
        "Control Group"
    }
    fn constraint_description() -> &'static str {
        r#"A group of controls, or of groups of controls."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog_group"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog:group"
    }
}
