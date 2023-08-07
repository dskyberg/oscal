/// Assessment Plan Terms and Conditions
/// Used to define various terms and conditions under which an assessment, described by the plan, can be performed. Each child part defines a different type of term or condition.
/// $id: #assembly_oscal-ap_assessment-plan_assessment-plan-terms-and-conditions_assessment-plan-terms-and-conditions
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentPart;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentPlanTermsAndConditions {
	pub parts: Option<Vec<AssessmentPart>>,
}
