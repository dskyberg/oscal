use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    BackMatter, Capability, DefinedComponent, ImportComponentDefinition, Metadata,
    SchemaConstraint, UUIDDatatype,
};
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

impl SchemaConstraint for ComponentDefinition {
    fn constraint_title() -> &'static str {
        "Component Definition"
    }
    fn constraint_description() -> &'static str {
        "A collection of component descriptions, which may optionally be grouped by capability."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_component-definition"
    }
    fn schema_path() -> &'static str {
        "definition"
    }
}
