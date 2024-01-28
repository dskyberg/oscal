/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/capability.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Capability {
}

impl SchemaConstraint for Capability {
    fn constraint_title() -> &'static str {
        "Capability"
    }
    fn constraint_description() -> &'static str {
        r#"A grouping of other components and/or capabilities."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_capability"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:capability"
    }
}
