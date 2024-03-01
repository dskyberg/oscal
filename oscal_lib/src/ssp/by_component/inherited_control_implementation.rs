/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/by_component.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, ResponsibleRole},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InheritedControlImplementation {
    pub uuid: UUIDDatatype,
    pub provided_uuid: Option<UUIDDatatype>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
}

impl SchemaConstraint for InheritedControlImplementation {
    fn constraint_title() -> &'static str {
        "Inherited Control Implementation"
    }
    fn constraint_description() -> &'static str {
        "Describes a control implementation inherited by a leveraging system."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_by-component_inherited-control-implementation"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:by-component:inherited-control-implementation"
    }
}
