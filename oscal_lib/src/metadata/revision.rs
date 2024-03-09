/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/revision.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

use super::{LastModified, Link, OscalVersion, Property, Published, Remarks, Version};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Revision {
    pub title: Option<String>,
    pub published: Option<Published>,
    pub last_modified: Option<LastModified>,
    pub version: Version,
    pub oscal_version: Option<OscalVersion>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Revision {
    fn schema_title() -> &'static str {
        "Revision History Entry"
    }
    fn schema_description() -> &'static str {
        r#"An entry in a sequential list of revisions to the containing document in reverse chronological order (i.e., most recent previous revision first)."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_revision")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:revision"
    }
}
