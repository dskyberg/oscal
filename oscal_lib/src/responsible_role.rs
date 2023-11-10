use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    MetadataLink, MetadataProperty, MetadataRemarks, SchemaConstraint, TokenDatatype, UUIDDatatype,
};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponsibleRole {
    pub role_id: TokenDatatype,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub party_uuids: Option<Vec<UUIDDatatype>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for ResponsibleRole {
    fn constraint_title() -> &'static str {
        "Responsible Role"
    }
    fn constraint_description() -> &'static str {
        "A reference to one or more roles with responsibility for performing a function relative to the containing object."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_responsible-role"
    }
    fn schema_path() -> &'static str {
        "responsible-role"
    }
}
