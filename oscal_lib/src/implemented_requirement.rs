use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    MetadataLink, MetadataProperty, MetadataRemarks, SchemaConstraint, TokenDatatype, UUIDDatatype,
};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementedRequirement {
    pub uuid: UUIDDatatype,
    pub control_id: TokenDatatype,
    pub description: String,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub set_parameters: Option<Vec<SetParameter>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub statements: Option<Vec<Statement>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for ImplementedRequirement {
    fn constraint_title() -> &'static str {
        "Control Implementation"
    }
    fn constraint_description() -> &'static str {
        "Describes how the containing component or capability implements an individual control."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_implemented-requirement"
    }
    fn schema_path() -> &'static str {
        "implemented-requirement"
    }
}
