/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/metadata.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

pub use address::Address;
pub use address_line::AddressLine;
pub use back_matter::BackMatter;
pub use document_id::DocumentId;
pub use hash::Hash;
pub use last_modified::LastModified;
pub use link::Link;
pub use location::Location;
pub use location_uuid::LocationUuid;
pub use oscal_version::OscalVersion;
pub use party::Party;
pub use property::Property;
pub use published::Published;
pub use remarks::Remarks;
pub use responsible_party::ResponsibleParty;
pub use responsible_role::ResponsibleRole;
pub use revision::Revision;
pub use role::Role;
pub use role_id::RoleId;
pub use telephone_number::TelephoneNumber;
pub use version::Version;

pub mod address;
pub mod address_line;
pub mod back_matter;
pub mod document_id;
pub mod hash;
pub mod last_modified;
pub mod link;
pub mod location;
pub mod location_uuid;
pub mod oscal_version;
pub mod party;
pub mod party_uuid;
pub mod property;
pub mod published;
pub mod remarks;
pub mod responsible_party;
pub mod responsible_role;
pub mod revision;
pub mod role;
pub mod role_id;
pub mod telephone_number;
pub mod version;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub title: String,
    pub published: Option<Published>,
    pub last_modified: LastModified,
    pub version: Version,
    pub oscal_version: OscalVersion,
    pub revisions: Option<Vec<Revision>>,
    pub document_ids: Option<Vec<DocumentId>>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub roles: Option<Vec<Role>>,
    pub locations: Option<Vec<Location>>,
    pub parties: Option<Vec<Party>>,
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    pub remark: Option<Vec<Remarks>>,
}

impl SchemaElement for Metadata {
    fn schema_title() -> &'static str {
        "Publication metadata"
    }
    fn schema_description() -> &'static str {
        r#"Provides information about the publication and availability of the containing document."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_metadata")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:metadata"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = include_str!("test/metadata.json");
        let result = serde_json::from_str::<Metadata>(json).expect("oops");
        dbg!(result);
    }
}
