/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/back_matter.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use self::back_matter_resource::BackMatterResource;

pub mod back_matter_resource;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatter {
    pub resources: Vec<BackMatterResource>,
}

impl SchemaConstraint for BackMatter {
    fn constraint_title() -> &'static str {
        "Back matter"
    }
    fn constraint_description() -> &'static str {
        r#"A collection of resources, which may be included directly or by reference."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter"
    }
}
