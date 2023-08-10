/// Security Impact Level
/// The overall level of expected impact resulting from unauthorized disclosure, modification, or loss of access to information.
/// $id: #assembly_oscal-ssp_security-impact-level
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SecurityImpactLevel {
	/// Security Objective: Integrity
	/// A target-level of integrity for the system, based on the sensitivity of information within the system.
	pub security_objective_integrity: StringDatatype,
	/// Security Objective: Confidentiality
	/// A target-level of confidentiality for the system, based on the sensitivity of information within the system.
	pub security_objective_confidentiality: StringDatatype,
	/// Security Objective: Availability
	/// A target-level of availability for the system, based on the sensitivity of information within the system.
	pub security_objective_availability: StringDatatype,
}
