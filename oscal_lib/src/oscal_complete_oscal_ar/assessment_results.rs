/// File name: ../oscal_lib/src/oscal_complete_oscal_ar/assessment_results.rs
/// pub use oscal_complete_oscal_ar::*;
///
/// pub mod oscal_complete_oscal_ar;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentResults {
}

impl SchemaConstraint for AssessmentResults {
    fn constraint_title() -> &'static str {
        "Security Assessment Results (SAR)"
    }
    fn constraint_description() -> &'static str {
        r#"Security assessment results, such as those provided by a FedRAMP assessor in the FedRAMP Security Assessment Report."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ar_assessment-results"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:assessment-results"
    }
}
