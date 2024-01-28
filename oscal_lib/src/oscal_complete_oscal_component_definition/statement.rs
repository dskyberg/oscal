/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/statement.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Statement {
}

impl SchemaConstraint for Statement {
    fn constraint_title() -> &'static str {
        "Control Statement Implementation"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies which statements within a control are addressed."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_statement"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:statement"
    }
}