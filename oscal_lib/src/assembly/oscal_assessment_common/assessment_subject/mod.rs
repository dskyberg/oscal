pub use subject_type::*;


pub mod subject_type;

/// Subject of Assessment
/// Identifies system elements being assessed, such as components, inventory items, and locations. In the assessment plan, this identifies a planned assessment subject. In the assessment results this is an actual assessment subject, and reflects any changes from the plan. exactly what will be the focus of this assessment. Any subjects not identified in this way are out-of-scope.
/// $id: #assembly_oscal-assessment-common_assessment-subject
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::SelectSubjectById;
use crate::assembly::oscal_catalog_common::IncludeAll;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentSubject {
	/// Include Subjects Description
	/// A human-readable description of the collection of subjects being included in this assessment.
	pub description: Option<String>,
	/// Subject Type
	/// Indicates the type of assessment subject, such as a component, inventory, item, location, or party represented by this selection statement.
	#[serde(rename = "type")]
	pub _type: SubjectType,
	pub include_subjects: Option<Vec<SelectSubjectById>>,
	pub remarks: Option<Remarks>,
	pub exclude_subjects: Option<Vec<SelectSubjectById>>,
	pub links: Option<Vec<Link>>,
	pub include_all: Option<IncludeAll>,
	pub props: Option<Vec<Property>>,
}
