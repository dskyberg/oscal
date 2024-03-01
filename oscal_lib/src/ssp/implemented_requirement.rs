/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/implemented_requirement.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::set_parameter::SetParameter,
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaConstraint, TokenDatatype, UUIDDatatype,
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

impl SchemaConstraint for ImplementedRequirement {
    fn constraint_title() -> &'static str {
        "Control-based Requirement"
    }
    fn constraint_description() -> &'static str {
        r#"Describes how the system satisfies the requirements of an individual control."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_implemented-requirement"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:implemented-requirement"
    }
}
