/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/party.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{EmailAddress, SchemaElement, StringDatatype, UUIDDatatype};

use self::external_identifier::ExternalIdentifier;

pub mod external_identifier;

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
    pub email_addresses: Option<Vec<EmailAddress>>,
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

impl SchemaElement for Party {
    fn schema_title() -> &'static str {
        "Party (organization or person)"
    }
    fn schema_description() -> &'static str {
        r#"A responsible entity which is either a person or an organization."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_party")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:party"
    }
}
