/// Task Dependency
/// Used to indicate that a task is dependent on another task.
/// $id: #assembly_oscal-assessment-common_task_task-dependency_task-dependency
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct TaskDependency {
	pub remarks: Option<Remarks>,
	/// Task Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a unique task.
	pub task_uuid: UuidDatatype,
}
