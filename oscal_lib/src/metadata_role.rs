use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    MetadataLink, MetadataProperty, MetadataRemarks, SchemaConstraint, StringDatatype,
    TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataRole {
    pub id: TokenDatatype,
    pub title: String,
    pub short_name: Option<StringDatatype>,
    pub description: Option<String>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for MetadataRole {
    fn constraint_title() -> &'static str {
        "Role"
    }
    fn constraint_description() -> &'static str {
        "Defines a function assumed or expected to be assumed by a party in a specific situation."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_role"
    }
    fn schema_path() -> &'static str {
        "role"
    }
}
