/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/defined_component.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefinedComponent {
}

impl SchemaConstraint for DefinedComponent {
    fn constraint_title() -> &'static str {
        "Component"
    }
    fn constraint_description() -> &'static str {
        r#"A defined component that can be part of an implemented system."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_defined-component"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:defined-component"
    }
}
