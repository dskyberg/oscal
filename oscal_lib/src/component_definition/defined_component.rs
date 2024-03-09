/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/defined_component.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::protocol::Protocol, metadata::Link, metadata::Property,
    metadata::Remarks, metadata::ResponsibleRole, SchemaElement, StringDatatype, UUIDDatatype,
};

use super::control_implementation::ControlImplementation;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefinedComponent {
    pub uuid: UUIDDatatype,
    /// "enum": [
    ///     "interconnection",
    ///     "software",
    ///     "hardware",
    ///     "service",
    ///     "policy",
    ///     "physical",
    ///     "process-procedure",
    ///     "plan",
    ///     "guidance",
    ///     "standard",
    ///     "validation"
    /// ]
    #[serde(rename = "type")]
    pub _type: StringDatatype,
    pub title: String,
    pub description: String,
    pub purpose: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub protocols: Option<Vec<Protocol>>,
    pub control_implementations: Option<Vec<ControlImplementation>>,
    pub remark: Option<Remarks>,
}

impl SchemaElement for DefinedComponent {
    fn schema_title() -> &'static str {
        "Component"
    }
    fn schema_description() -> &'static str {
        r#"A defined component that can be part of an implemented system."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:defined-component"
    }
}
