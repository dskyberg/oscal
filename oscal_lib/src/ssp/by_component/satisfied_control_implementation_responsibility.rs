/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/by_component.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaElement, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SatisfiedControlImplementationResponsibility {
    pub uuid: UUIDDatatype,
    pub responsibility_uuid: Option<UUIDDatatype>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for SatisfiedControlImplementationResponsibility {
    fn schema_title() -> &'static str {
        "Satisfied Control Implementation Responsibility"
    }
    fn schema_description() -> &'static str {
        "Describes how this system satisfies a responsibility imposed by a leveraged system."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:by-component:satisfied-control-implementation-responsibility"
    }
}
