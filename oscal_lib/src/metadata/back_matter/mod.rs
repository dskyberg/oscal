/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/back_matter.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

use self::resource::Resource;

pub mod resource;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatter {
    pub resources: Vec<Resource>,
}

impl SchemaElement for BackMatter {
    fn schema_title() -> &'static str {
        "Back matter"
    }
    fn schema_description() -> &'static str {
        r#"A collection of resources, which may be included directly or by reference."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_back-matter")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter"
    }
}
