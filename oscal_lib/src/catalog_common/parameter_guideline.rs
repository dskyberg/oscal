/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/parameter_guideline.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterGuideline {
    pub prose: String,
}

impl SchemaElement for ParameterGuideline {
    fn schema_title() -> &'static str {
        "Guideline"
    }
    fn schema_description() -> &'static str {
        r#"A prose statement that provides a recommendation for the use of a parameter."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-catalog-common_parameter-guideline")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:parameter-guideline"
    }
}
