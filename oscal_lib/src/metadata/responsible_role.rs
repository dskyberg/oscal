/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/responsible_role.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, TokenDatatype};

use super::{party_uuid::PartyUuid, Link, Property, Remarks};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponsibleRole {
    pub role_id: TokenDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub party_uuids: Option<Vec<PartyUuid>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ResponsibleRole {
    fn schema_title() -> &'static str {
        "Responsible Role"
    }
    fn schema_description() -> &'static str {
        r#"A reference to one or more roles with responsibility for performing a function relative to the containing object."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_responsible-role")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:responsible-role"
    }
}
