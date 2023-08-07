/// Actor Type
/// The kind of actor.
/// $id: #assembly_oscal-assessment-common_origin-actor_actor-type_actor-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ActorType {
	// orig: tool
	Tool,
	// orig: assessment-platform
	AssessmentPlatform,
	// orig: party
	Party,
}
