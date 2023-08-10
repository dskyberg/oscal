pub use objective_status::*;
pub use finding_target_type::*;


pub mod objective_status;
pub mod finding_target_type;

/// Objective Status
/// Captures an assessor's conclusions regarding the degree to which an objective is satisfied.
/// $id: #assembly_oscal-assessment-common_finding-target
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::ImplementationStatus;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct FindingTarget {
	pub links: Option<Vec<Link>>,
	/// Finding Target Identifier Reference
	/// A machine-oriented identifier reference for a specific target qualified by the type.
	pub target_id: TokenDatatype,
	/// Objective Status
	/// A determination of if the objective is satisfied or not within a given system.
	pub status: ObjectiveStatus,
	/// Objective Status Description
	/// A human-readable description of the assessor's conclusions regarding the degree to which an objective is satisfied.
	pub description: Option<String>,
	pub implementation_status: Option<ImplementationStatus>,
	/// Objective Status Title
	/// The title for this objective status.
	pub title: Option<String>,
	/// Finding Target Type
	/// Identifies the type of the target.
	#[serde(rename = "type")]
	pub _type: FindingTargetType,
	pub remarks: Option<Remarks>,
	pub props: Option<Vec<Property>>,
}
