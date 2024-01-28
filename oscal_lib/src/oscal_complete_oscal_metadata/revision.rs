/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/revision.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Revision {
    pub title: Option<String>,
    /// "#field_oscal-metadata_published"
    pub published: Option<super::published::Published>,
    /// "#field_oscal-metadata_last-modified"
    pub last_modified: Option<super::last_modified::LastModified>,
    /// "#field_oscal-metadata_version"
    pub version: super::version::Version,
    /// "#field_oscal-metadata_oscal-version"
    pub oscal_version: Option<super::oscal_version::OscalVersion>,
    /// #assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<super::remarks::Remarks>,
}

impl SchemaConstraint for Revision {
    fn constraint_title() -> &'static str {
        "Revision History Entry"
    }
    fn constraint_description() -> &'static str {
        r#"An entry in a sequential list of revisions to the containing document in reverse chronological order (i.e., most recent previous revision first)."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_revision"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:revision"
    }
}
