/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/incorporates_component.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IncorporatesComponent {
    pub component_uuid: UUIDDatatype,
    pub description: String,
}

impl SchemaElement for IncorporatesComponent {
    fn schema_title() -> &'static str {
        "Incorporates Component"
    }
    fn schema_description() -> &'static str {
        r#"TBD"#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:incorporates-component"
    }
}
