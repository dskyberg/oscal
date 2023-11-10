use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IncorporatesComponent {
    pub component_uuid: UUIDDatatype,
    pub description: String,
}

impl SchemaConstraint for IncorporatesComponent {
    fn constraint_title() -> &'static str {
        "Incorporates Component"
    }
    fn constraint_description() -> &'static str {
        "TBD"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_incorporates-component"
    }
    fn schema_path() -> &'static str {
        "incorporates-component"
    }
}
