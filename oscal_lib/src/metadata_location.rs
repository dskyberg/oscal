///562
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    EmailAddressDatatype, MetadataAddress, MetadataLink, MetadataProperty, MetadataRemarks,
    SchemaConstraint, TelephoneNumber, URIDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataLocation {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub address: MetadataAddress,
    // "$ref": "#field_oscal-metadata_email-address"
    pub email_address: Option<EmailAddressDatatype>,
    pub telephone_numbers: Option<Vec<TelephoneNumber>>,
    pub urls: Option<Vec<URIDatatype>>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for MetadataLocation {
    fn constraint_title() -> &'static str {
        "Location"
    }
    fn constraint_description() -> &'static str {
        "A location, with associated metadata that can be referenced."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_location"
    }
    fn schema_path() -> &'static str {
        "location"
    }
}
