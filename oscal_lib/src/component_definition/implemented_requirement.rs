/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/implemented_requirement.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::set_parameter::SetParameter,
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaElement, TokenDatatype, UUIDDatatype,
};

use super::statement::Statement;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementedRequirement {
    pub uuid: UUIDDatatype,
    pub control_id: TokenDatatype,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub set_parameters: Option<Vec<SetParameter>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub statements: Option<Vec<Statement>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ImplementedRequirement {
    fn schema_title() -> &'static str {
        "Control Implementation"
    }
    fn schema_description() -> &'static str {
        r#"Describes how the containing component or capability implements an individual control."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:implemented-requirement"
    }
}
