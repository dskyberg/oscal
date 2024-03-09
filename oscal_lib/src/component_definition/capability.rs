/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/capability.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::Link, metadata::Property, metadata::Remarks, SchemaElement, StringDatatype,
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

impl SchemaElement for Capability {
    fn schema_title() -> &'static str {
        "Capability"
    }
    fn schema_description() -> &'static str {
        r#"A grouping of other components and/or capabilities."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:capability"
    }
}
