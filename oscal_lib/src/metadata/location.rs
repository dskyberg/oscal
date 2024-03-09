/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/location.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Address, Link, Property, Remarks, SchemaElement, TelephoneNumber};
use crate::{EmailAddress, URIDatatype, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub address: Option<Address>,
    pub email_address: Option<EmailAddress>,
    pub telephone_numbers: Option<Vec<TelephoneNumber>>,
    pub urls: Option<Vec<URIDatatype>>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Location {
    fn schema_title() -> &'static str {
        "Location"
    }
    fn schema_description() -> &'static str {
        r#"A location, with associated metadata that can be referenced."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_location")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:location"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = r##"        {
            "uuid": "1bd641ff-54a7-40d5-acc9-82eed9d22d4a",
            "title": "Okta HQ North",
            "address": {
                "addr-lines": [
                    "100 First Street",
                    "6th Floor"
                ],
                "city": "San Francisco",
                "state": "CA",
                "postal-code": "94105"
            },
            "remarks": "Okta SF Office"
        }"##;

        let location = serde_json::from_str::<Location>(json).expect("failed");
        dbg!(&location);
    }
}
