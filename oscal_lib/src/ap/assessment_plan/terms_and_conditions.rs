use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assessment_common::assessment_part::AssessmentPart;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TermsAndConditions {
    pub parts: Option<Vec<AssessmentPart>>,
}
