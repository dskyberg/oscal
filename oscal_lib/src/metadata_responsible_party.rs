use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    MetadataLink, MetadataProperty, MetadataRemarks, SchemaConstraint, TokenDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataResponsibleParty {
    pub role_id: TokenDatatype,
    pub party_uuids: Vec<UUIDDatatype>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for MetadataResponsibleParty {
    fn constraint_title() -> &'static str {
        "Responsible Party"
    }
    fn constraint_description() -> &'static str {
        "A reference to a set of organizations or persons that have responsibility for performing a referenced role in the context of the containing object."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_responsible-party"
    }
    fn schema_path() -> &'static str {
        "responsible-party"
    }
}
