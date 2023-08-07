/// Finding Target Type
/// Identifies the type of the target.
/// $id: #assembly_oscal-assessment-common_finding-target_finding-target-type_finding-target-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum FindingTargetType {
	// orig: statement-id
	StatementId,
	// orig: objective-id
	ObjectiveId,
}
