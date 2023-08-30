pub use local_definitions::*;
pub use assessment_plan_terms_and_conditions::*;


pub mod local_definitions;
pub mod assessment_plan_terms_and_conditions;

/// Security Assessment Plan (SAP)
/// An assessment plan, such as those provided by a FedRAMP assessor.
/// $id: #assembly_oscal-ap_assessment-plan
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentAssets;
use crate::assembly::oscal_assessment_common::AssessmentSubject;
use crate::assembly::oscal_assessment_common::ImportSsp;
use crate::assembly::oscal_assessment_common::ReviewedControls;
use crate::assembly::oscal_assessment_common::Task;
use crate::assembly::oscal_metadata::BackMatter;
use crate::assembly::oscal_metadata::Metadata;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentPlan {
	pub tasks: Option<Vec<Task>>,
	/// Assessment Plan Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this assessment plan in this or other OSCAL instances. The locally defined UUID of the assessment plan can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub metadata: Metadata,
	pub assessment_subjects: Option<Vec<AssessmentSubject>>,
	/// Local Definitions
	/// Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
	pub local_definitions: Option<LocalDefinitions>,
	pub assessment_assets: Option<AssessmentAssets>,
	/// Assessment Plan Terms and Conditions
	/// Used to define various terms and conditions under which an assessment, described by the plan, can be performed. Each child part defines a different type of term or condition.
	pub terms_and_conditions: Option<AssessmentPlanTermsAndConditions>,
	pub reviewed_controls: ReviewedControls,
	pub import_ssp: ImportSsp,
	pub back_matter: Option<BackMatter>,
}
