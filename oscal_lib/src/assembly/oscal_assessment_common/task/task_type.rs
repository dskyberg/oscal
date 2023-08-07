/// Task Type
/// The type of task.
/// $id: #assembly_oscal-assessment-common_task_task-type_task-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum TaskType {
	// orig: milestone
	Milestone,
	// orig: action
	Action,
}
