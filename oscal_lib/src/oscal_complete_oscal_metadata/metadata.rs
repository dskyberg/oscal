/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/metadata.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub title: String,
    /// "$ref": "#field_oscal-metadata_published"
    pub published: Option<super::published::Published>,
    /// "$ref": "#field_oscal-metadata_last-modified"
    pub last_modified: super::last_modified::LastModified,
    /// "$ref": "#field_oscal-metadata_version"
    pub version: super::version::Version,
    /// "$ref": "#field_oscal-metadata_oscal-version"
    pub oscal_version: super::oscal_version::OscalVersion,
    /// "$ref": "#assembly_oscal-metadata_revision"
    pub revisions: Option<Vec<super::revision::Revision>>,
    ///  "$ref": "#field_oscal-metadata_document-id"
    pub document_ids: Option<Vec<super::document_id::DocumentId>>,
    /// "$ref": "#assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
    /// "#assembly_oscal-metadata_role"
    pub roles: Option<Vec<super::role::Role>>,
    /// "#assembly_oscal-metadata_party"
    pub parties: Option<Vec<super::party::Party>>,
    ///"#assembly_oscal-metadata_responsible-party"
    pub responsible_parties: Option<Vec<super::responsible_party::ResponsibleParty>>,
    /// "#field_oscal-metadata_remarks"
    pub remark: Option<Vec<super::remarks::Remarks>>,
}

impl SchemaConstraint for Metadata {
    fn constraint_title() -> &'static str {
        "Publication metadata"
    }
    fn constraint_description() -> &'static str {
        r#"Provides information about the publication and availability of the containing document."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_metadata"
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
