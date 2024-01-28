/// File name: ../oscal_lib/src/oscal_complete_oscal_ar/result.rs
/// pub use oscal_complete_oscal_ar::*;
///
/// pub mod oscal_complete_oscal_ar;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Result {
}

impl SchemaConstraint for Result {
    fn constraint_title() -> &'static str {
        "Assessment Result"
    }
    fn constraint_description() -> &'static str {
        r#"Used by the assessment results and POA&M. In the assessment results, this identifies all of the assessment observations and findings, initial and residual risks, deviations, and disposition. In the POA&M, this identifies initial and residual risks, deviations, and disposition."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ar_result"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:result"
    }
}
