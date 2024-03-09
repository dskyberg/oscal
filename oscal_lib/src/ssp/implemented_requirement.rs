/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/implemented_requirement.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::set_parameter::SetParameter,
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaElement, TokenDatatype, UUIDDatatype,
};

use super::{by_component::ByComponent, statement::Statement};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementedRequirement {
    pub uuid: UUIDDatatype,
    pub control_id: TokenDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub set_parameters: Option<Vec<SetParameter>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub statements: Option<Vec<Statement>>,
    pub by_components: Option<Vec<ByComponent>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ImplementedRequirement {
    fn schema_title() -> &'static str {
        "Control-based Requirement"
    }
    fn schema_description() -> &'static str {
        r#"Describes how the system satisfies the requirements of an individual control."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_implemented-requirement")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:implemented-requirement"
    }
}
