pub use identified_subject::*;


pub mod identified_subject;

/// Task Reference
/// Identifies an individual task for which the containing object is a consequence of.
/// $id: #assembly_oscal-assessment-common_related-task
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentSubject;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleParty;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct RelatedTask {
	pub links: Option<Vec<Link>>,
	pub responsible_parties: Option<Vec<ResponsibleParty>>,
	pub props: Option<Vec<Property>>,
	/// Identified Subject
	/// Used to detail assessment subjects that were identfied by this task.
	pub identified_subject: Option<IdentifiedSubject>,
	pub remarks: Option<Remarks>,
	pub subjects: Option<Vec<AssessmentSubject>>,
	/// Task Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a unique task.
	pub task_uuid: UuidDatatype,
}
