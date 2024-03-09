/// File name: ../oscal_lib/src/oscal_complete_oscal_component_definition/component_definition.rs
/// pub use oscal_complete_oscal_component_definition::*;
///
/// pub mod oscal_complete_oscal_component_definition;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{BackMatter, Metadata},
    SchemaElement, UUIDDatatype,
};

use self::{
    capability::Capability, defined_component::DefinedComponent,
    import_component_definition::ImportComponentDefinition,
};

pub mod capability;
pub mod control_implementation;
pub mod defined_component;
pub mod implemented_requirement;
pub mod import_component_definition;
pub mod incorporates_component;
pub mod statement;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ComponentDefinition {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub import_component_definitions: Option<Vec<ImportComponentDefinition>>,
    pub components: Option<Vec<DefinedComponent>>,
    pub capabilities: Option<Vec<Capability>>,
    pub back_matter: Option<BackMatter>,
}

impl SchemaElement for ComponentDefinition {
    fn schema_title() -> &'static str {
        "Component Definition"
    }
    fn schema_description() -> &'static str {
        r#"A collection of component descriptions, which may optionally be grouped by capability."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-component-definition_component-definition")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-component-definition:component-definition"
    }
}
