/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/party.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, URIDatatype, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExternalIdentifier {
    /// enum: ["http://orcid.org/"]
    pub schema: URIDatatype,
    pub id: StringDatatype,
}

impl SchemaConstraint for ExternalIdentifier {
    fn constraint_title() -> &'static str {
        "Party External Identifier"
    }

    fn constraint_description() -> &'static str {
        "An identifier for a person or organization using a designated scheme. e.g. an Open Researcher and Contributor ID (ORCID)"
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_party:external-ids"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:party/external-ids"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Party {
    pub uuid: UUIDDatatype,
    /// enum: [
    /// "person",
    /// "organization",
    /// ]
    #[serde(rename = "type")]
    pub _type: StringDatatype,
    pub name: Option<StringDatatype>,
    pub short_name: Option<StringDatatype>,
    pub external_ids: Option<Vec<ExternalIdentifier>>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
    /// "#field_oscal-metadata_email-address"
    pub email_addresses: Option<Vec<super::email_address::EmailAddress>>,
    /// "#field_oscal-metadata_telephone-number"
    pub telephone_numbers: Option<Vec<super::telephone_number::TelephoneNumber>>,
    /// "#assembly_oscal-metadata_address"
    pub addresses: Option<Vec<super::address::Address>>,
    /// "#field_oscal-metadata_location-uuid"
    pub location_uuids: Option<Vec<super::location_uuid::LocationUuid>>,
    pub member_of_organizations: Option<Vec<UUIDDatatype>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<super::remarks::Remarks>,
}

impl SchemaConstraint for Party {
    fn constraint_title() -> &'static str {
        "Party (organization or person)"
    }
    fn constraint_description() -> &'static str {
        r#"A responsible entity which is either a person or an organization."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_party"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:party"
    }
}
