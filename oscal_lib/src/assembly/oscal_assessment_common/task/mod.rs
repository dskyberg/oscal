pub use associated_activity::*;
pub use task_dependency::*;
pub use event_timing::*;
pub use task_type::*;


pub mod associated_activity;
pub mod task_dependency;
pub mod event_timing;
pub mod task_type;

/// Task
/// Represents a scheduled event or milestone, which may be associated with a series of assessment actions.
/// $id: #assembly_oscal-assessment-common_task
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentSubject;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Task {
	pub props: Option<Vec<Property>>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	/// Task Type
	/// The type of task.
	#[serde(rename = "type")]
	pub _type: TaskType,
	pub dependencies: Option<Vec<TaskDependency>>,
	pub associated_activities: Option<Vec<AssociatedActivity>>,
	pub remarks: Option<Remarks>,
	/// Task Description
	/// A human-readable description of this task.
	pub description: Option<String>,
	/// Event Timing
	/// The timing under which the task is intended to occur.
	pub timing: Option<EventTiming>,
	pub subjects: Option<Vec<AssessmentSubject>>,
	/// Task Title
	/// The title for this task.
	pub title: String,
	/// Task Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this task elsewhere in this or other OSCAL instances. The locally defined UUID of the task can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub tasks: Option<Vec<Task>>,
	pub links: Option<Vec<Link>>,
}
