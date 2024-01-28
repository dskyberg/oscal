/// File name: ../oscal_lib/src/oscal_complete_oscal_ap/assessment_plan.rs
/// pub use oscal_complete_oscal_ap::*;
///
/// pub mod oscal_complete_oscal_ap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPlan {
}

impl SchemaConstraint for AssessmentPlan {
    fn constraint_title() -> &'static str {
        "Security Assessment Plan (SAP)"
    }
    fn constraint_description() -> &'static str {
        r#"An assessment plan, such as those provided by a FedRAMP assessor."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ap_assessment-plan"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ap:assessment-plan"
    }
}
