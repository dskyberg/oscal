use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaConstraint, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaskDependency {
    pub task_uuid: UUIDDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for TaskDependency {
    fn constraint_title() -> &'static str {
        "Task Dependency"
    }
    fn constraint_description() -> &'static str {
        "Used to indicate that a task is dependent on another task."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:associated-activity"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:associated-activity"
    }
}
