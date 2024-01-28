/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/location.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, URIDatatype, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    /// "#assembly_oscal-metadata_address"
    pub address: Option<super::address::Address>,
    /// "#field_oscal-metadata_email-address"
    pub email_address: Option<super::email_address::EmailAddress>,
    /// "#field_oscal-metadata_telephone-number"
    pub telephone_numbers: Option<Vec<super::telephone_number::TelephoneNumber>>,
    /// #/definitions/URIDatatype"
    pub urls: Option<Vec<URIDatatype>>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
    /// "#assembly_oscal-metadata_link"
    pub remark: Option<super::remarks::Remarks>,
}

impl SchemaConstraint for Location {
    fn constraint_title() -> &'static str {
        "Location"
    }
    fn constraint_description() -> &'static str {
        r#"A location, with associated metadata that can be referenced."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_location"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:location"
    }
}
