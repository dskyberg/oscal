/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/responsible_party.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponsibleParty {
    pub role_id: TokenDatatype,
    /// "#field_oscal-metadata_party-uuid"
    pub party_uuids: Vec<super::party_uuid::PartyUuid>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<Vec<super::remarks::Remarks>>,
}

impl SchemaConstraint for ResponsibleParty {
    fn constraint_title() -> &'static str {
        "Responsible Party"
    }
    fn constraint_description() -> &'static str {
        r#"A reference to a set of organizations or persons that have responsibility for performing a referenced role in the context of the containing object."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_responsible-party"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:responsible-party"
    }
}
