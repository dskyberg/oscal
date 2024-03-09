use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{assessment_common::assessment_part::AssessmentPart, SchemaElement};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TermsAndConditions {
    pub parts: Option<Vec<AssessmentPart>>,
}

impl SchemaElement for TermsAndConditions {
    fn schema_title() -> &'static str {
        "Assessment Plan Terms and Conditions"
    }
    fn schema_description() -> &'static str {
        "Used to define various terms and conditions under which an assessment, described by the plan, can be performed. Each child part defines a different type of term or condition."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ap:assessment-plan/terms-and-conditions"
    }
}
