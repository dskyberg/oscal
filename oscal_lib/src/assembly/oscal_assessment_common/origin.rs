/// Origin
/// Identifies the source of the finding, such as a tool, interviewed person, or activity.
/// $id: #assembly_oscal-assessment-common_origin
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::OriginActor;
use crate::assembly::oscal_assessment_common::RelatedTask;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Origin {
	pub actors: Vec<OriginActor>,
	pub related_tasks: Option<Vec<RelatedTask>>,
}
