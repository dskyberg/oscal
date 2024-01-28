/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/responsible_role.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponsibleRole {
    pub role_id: TokenDatatype,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
    /// "#field_oscal-metadata_party-uuid"
    pub party_uuids: Option<Vec<super::party_uuid::PartyUuid>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<super::remarks::Remarks>,
}

impl SchemaConstraint for ResponsibleRole {
    fn constraint_title() -> &'static str {
        "Responsible Role"
    }
    fn constraint_description() -> &'static str {
        r#"A reference to one or more roles with responsibility for performing a function relative to the containing object."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_responsible-role"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:responsible-role"
    }
}
