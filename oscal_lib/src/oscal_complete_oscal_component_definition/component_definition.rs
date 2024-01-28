/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/component_definition.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ComponentDefinition {
}

impl SchemaConstraint for ComponentDefinition {
    fn constraint_title() -> &'static str {
        "Component Definition"
    }
    fn constraint_description() -> &'static str {
        r#"A collection of component descriptions, which may optionally be grouped by capability."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_component-definition"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:component-definition"
    }
}
