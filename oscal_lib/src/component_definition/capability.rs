/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/capability.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::Link, metadata::Property, metadata::Remarks, SchemaConstraint, StringDatatype,
    UUIDDatatype,
};

use super::{
    control_implementation::ControlImplementation, incorporates_component::IncorporatesComponent,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Capability {
    pub uuid: UUIDDatatype,
    pub name: StringDatatype,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub incorporates_components: Option<Vec<IncorporatesComponent>>,
    pub control_implementations: Option<Vec<ControlImplementation>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for Capability {
    fn constraint_title() -> &'static str {
        "Capability"
    }
    fn constraint_description() -> &'static str {
        r#"A grouping of other components and/or capabilities."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_capability"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:capability"
    }
}
