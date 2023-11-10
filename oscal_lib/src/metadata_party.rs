use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    EmailAddressDatatype, MetadataAddress, MetadataLink, MetadataProperty, MetadataRemarks,
    SchemaConstraint, StringDatatype, TelephoneNumber, URIDatatype, UUIDDatatype,
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PartyExternalIdentifier {
    id: StringDatatype,
    scheme: URIDatatype,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "party/external-id"}
    ],
    "allowed-values": [
        "http://orcid.org/"
        "https://orcid.org/"
    ]
}
 */
impl SchemaConstraint for PartyExternalIdentifier {
    fn constraint_title() -> &'static str {
        "Party External Identifier"
    }
    fn constraint_description() -> &'static str {
        "An identifier for a person or organization using a designated scheme. e.g. an Open Researcher and Contributor ID (ORCID)"
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "external-id"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataParty {
    pub uuid: UUIDDatatype,
    #[serde(rename = "type")]
    pub type_: StringDatatype,
    pub name: Option<StringDatatype>,
    pub short_name: Option<StringDatatype>,
    pub external_ids: Option<Vec<PartyExternalIdentifier>>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub email_addresses: Option<Vec<EmailAddressDatatype>>,
    pub telephone_numbers: Option<Vec<TelephoneNumber>>,
    pub addresses: Option<Vec<MetadataAddress>>,
    pub location_uuids: Option<Vec<UUIDDatatype>>,
    pub member_of_organizations: Option<Vec<UUIDDatatype>>,
    pub remarks: Option<MetadataRemarks>,
}

/*
"constraint": {
    "title": "Party Type",
    "description": "A category describing the kind of party the object describes.",
    "bindings": [
        {"pattern": "party/type"}
    ]
    "allowed-values": [
        "person",
        "organization"
    ]
}
*/

impl SchemaConstraint for MetadataParty {
    fn constraint_title() -> &'static str {
        "Party (organization or person)"
    }
    fn constraint_description() -> &'static str {
        "A responsible entity which is either a person or an organization."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_party"
    }
    fn schema_path() -> &'static str {
        "party"
    }
}
