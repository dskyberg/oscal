/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/control_implementation.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::implemented_requirement::ImplementedRequirement;

use crate::{
    implementation_common::set_parameter::SetParameter, metadata::Link, metadata::Property,
    SchemaElement, URIReferenceDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlImplementation {
    pub uuid: UUIDDatatype,
    pub source: URIReferenceDatatype,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub set_parameters: Option<Vec<SetParameter>>,
    pub implemented_requirements: Vec<ImplementedRequirement>,
}

impl SchemaElement for ControlImplementation {
    fn schema_title() -> &'static str {
        "Control Implementation Set"
    }
    fn schema_description() -> &'static str {
        r#"Defines how the component or capability supports a set of controls."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:control-implementation"
    }
}
