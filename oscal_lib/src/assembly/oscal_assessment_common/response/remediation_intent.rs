/// Remediation Intent
/// Identifies whether this is a recommendation, such as from an assessor or tool, or an actual plan accepted by the system owner.
/// $id: #assembly_oscal-assessment-common_response_remediation-intent_remediation-intent
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum RemediationIntent {
	// orig: recommendation
	Recommendation,
	// orig: planned
	Planned,
	// orig: completed
	Completed,
}
