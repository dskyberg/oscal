use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    DateTimeWithTimezoneDatatype, MetadataLink, MetadataProperty, MetadataRemarks,
    SchemaConstraint, StringDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataRevision {
    pub title: Option<String>,
    pub published: Option<DateTimeWithTimezoneDatatype>,
    pub last_modified: Option<DateTimeWithTimezoneDatatype>,
    pub version: StringDatatype,
    pub oscal_version: Option<StringDatatype>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for MetadataRevision {
    fn constraint_title() -> &'static str {
        "Revision History Entry"
    }
    fn constraint_description() -> &'static str {
        "An entry in a sequential list of revisions to the containing document in reverse chronological order (i.e., most recent previous revision first)."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_revision"
    }
    fn schema_path() -> &'static str {
        "revision"
    }
}
