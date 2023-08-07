pub use resource::*;

pub mod resource;

/// Back matter
/// A collection of resources, which may be included directly or by reference.
/// $id: #assembly_oscal-metadata_back-matter
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatter {
    pub resources: Option<Vec<Resource>>,
}
