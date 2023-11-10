use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{MetadataLink, MetadataProperty, SchemaConstraint, URIReferenceDatatype, UUIDDatatype};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlImplentation {
    pub uuid: UUIDDatatype,
    pub source: URIReferenceDatatype,
    pub description: String,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub set_parameters: Option<Vec<SetParameter>>,
    pub implented_requirements: Option<Vec<ImplentedRequirement>>,
}

impl SchemaConstraint for ControlImplentation {
    fn constraint_title() -> &'static str {
        "Control Implementation Set"
    }
    fn constraint_description() -> &'static str {
        "Defines how the component or capability supports a set of controls."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_control-implementation"
    }
    fn schema_path() -> &'static str {
        "control-implemtation"
    }
}
