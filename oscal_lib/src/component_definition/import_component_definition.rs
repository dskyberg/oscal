/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/import_component_definition.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportComponentDefinition {
    pub href: URIReferenceDatatype,
}

impl SchemaElement for ImportComponentDefinition {
    fn schema_title() -> &'static str {
        "Import Component Definition"
    }
    fn schema_description() -> &'static str {
        r#"Loads a component definition from another resource."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:import-component-definition"
    }
}
