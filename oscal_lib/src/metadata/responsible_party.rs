/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/responsible_party.rs
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
pub struct ResponsibleParty {
    pub role_id: TokenDatatype,
    pub party_uuids: Vec<PartyUuid>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ResponsibleParty {
    fn schema_title() -> &'static str {
        "Responsible Party"
    }
    fn schema_description() -> &'static str {
        r#"A reference to a set of organizations or persons that have responsibility for performing a referenced role in the context of the containing object."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_responsible-party")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:responsible-party"
    }
}
