/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/include_all.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

/// include-all is basically an empty
/// object that is treated as a flag.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IncludeAll {}

impl SchemaElement for IncludeAll {
    fn schema_title() -> &'static str {
        "Include All"
    }
    fn schema_description() -> &'static str {
        r#"Include all controls from the imported catalog or profile resources."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-catalog-common_include-all")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:include-all"
    }
}
