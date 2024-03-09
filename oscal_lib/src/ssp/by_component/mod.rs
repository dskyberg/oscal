/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/by_component.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::{
        implementation_status::ImplementationStatus, set_parameter::SetParameter,
    },
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaElement, UUIDDatatype,
};

use self::{
    export::Export, inherited_control_implementation::InheritedControlImplementation,
    satisfied_control_implementation_responsibility::SatisfiedControlImplementationResponsibility,
};

pub mod export;
pub mod inherited_control_implementation;
pub mod satisfied_control_implementation_responsibility;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ByComponent {
    pub component_uuid: UUIDDatatype,
    pub uuid: UUIDDatatype,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub set_parameters: Option<Vec<SetParameter>>,
    pub implementation_status: Option<ImplementationStatus>,
    pub export: Option<Export>,
    pub inherited: Option<Vec<InheritedControlImplementation>>,
    pub satisfied: Option<Vec<SatisfiedControlImplementationResponsibility>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ByComponent {
    fn schema_title() -> &'static str {
        "Component Control Implementation"
    }
    fn schema_description() -> &'static str {
        r#"Defines how the referenced component implements a set of controls."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_by-component")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:by-component"
    }
}
