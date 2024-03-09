use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaElement, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaskDependency {
    pub task_uuid: UUIDDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for TaskDependency {
    fn schema_title() -> &'static str {
        "Task Dependency"
    }
    fn schema_description() -> &'static str {
        "Used to indicate that a task is dependent on another task."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:associated-activity"
    }
}
