/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/by_component.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ByComponent {
}

impl SchemaConstraint for ByComponent {
    fn constraint_title() -> &'static str {
        "Component Control Implementation"
    }
    fn constraint_description() -> &'static str {
        r#"Defines how the referenced component implements a set of controls."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_by-component"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:by-component"
    }
}
