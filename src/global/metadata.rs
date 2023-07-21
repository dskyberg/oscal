use chrono::prelude::*;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::{
    DocumentIds, Locations, Parties, Properties, ResponsibleParties, Revisions, Roles, VersionVar,
};

/// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    /// A name given to the document, which may be used by a tool for display and navigation.
    pub title: String,
    ///  The date and time the document was published. The date-time value must be formatted according to RFC 3339 with full time and time zone included.
    pub published: Option<DateTime<Utc>>,
    /// The date and time the document was last modified. The date-time value must be formatted according to RFC 3339 with full time and time zone included.
    pub last_modified: DateTime<Utc>,
    /// A string used to distinguish the current version of the document from other previous (and future) versions.
    pub version: VersionVar,
    /// The OSCAL model version the document was authored against.
    pub oscal_version: Version,
    /// An entry in a sequential list of revisions to the containing document in reverse chronological order (i.e., most recent previous revision first).
    pub revisions: Option<Revisions>,
    /// A document identifier qualified by an identifier scheme.
    pub document_ids: Option<DocumentIds>,
    /// An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values.
    pub props: Option<Properties>,
    /// A reference to a local or remote resource
    pub links: Option<Vec<String>>, // TODO: validate as URIs
    /// Defines a function assumed or expected to be assumed by a party in a specific situation.
    pub roles: Option<Roles>,
    /// A location, with associated metadata that can be referenced.
    pub locations: Option<Locations>,
    /// A responsible entity which is either a person or an organization.
    pub parties: Option<Parties>,
    /// A reference to a set of organizations or persons that have responsibility for performing a referenced role in the context of the containing object.
    pub responsible_parties: Option<ResponsibleParties>,
    /// A reference to a set of organizations or persons that have responsibility for performing a referenced role in the context of the containing object.
    pub remarks: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let m = Metadata {
            title: "My Title".to_string(),
            published: None,
            last_modified: "2023-07-16T0:0:0-05:00"
                .parse::<DateTime<Utc>>()
                .expect("bad date"),
            version: VersionVar::from("just a string"),
            oscal_version: Version::new(1, 0, 0),
            revisions: None,
            document_ids: None,
            props: None,
            links: None,
            roles: None,
            locations: None,
            parties: None,
            responsible_parties: None,
            remarks: None,
        };

        let json = serde_json::to_string(&m).expect("fail!");
        println!("{}", &json);
    }

    #[test]
    fn test_deserialize() {
        let json = include_str!("../../tests/metadata_1.json");
        let m: Metadata = serde_json::from_str(json).expect("nope");
        dbg!(&m);
    }
}
