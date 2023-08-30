pub use objective_status::*;


pub mod objective_status;

/// Objective Status
/// Captures an assessor's conclusions regarding the degree to which an objective is satisfied.
/// $id: #assembly_oscal-assessment-common_finding-target
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::ImplementationStatus;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::StringDatatype;
use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct FindingTarget {
	/// Objective Status Description
	/// A human-readable description of the assessor's conclusions regarding the degree to which an objective is satisfied.
	pub description: Option<String>,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	/// Objective Status
	/// A determination of if the objective is satisfied or not within a given system.
	pub status: ObjectiveStatus,
	/// Objective Status Title
	/// The title for this objective status.
	pub title: Option<String>,
	pub props: Option<Vec<Property>>,
	/// Finding Target Identifier Reference
	/// A machine-oriented identifier reference for a specific target qualified by the type.
	pub target_id: TokenDatatype,
	/// Finding Target Type
	/// Identifies the type of the target.
	#[serde(rename = "type")]
	pub _type: StringDatatype,
	pub implementation_status: Option<ImplementationStatus>,
}
