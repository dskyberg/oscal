/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/control_implementation.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlImplementation {
}

impl SchemaConstraint for ControlImplementation {
    fn constraint_title() -> &'static str {
        "Control Implementation Set"
    }
    fn constraint_description() -> &'static str {
        r#"Defines how the component or capability supports a set of controls."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_control-implementation"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:control-implementation"
    }
}
