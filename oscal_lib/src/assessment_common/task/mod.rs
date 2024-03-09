/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/task.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{link, property, remarks, responsible_role},
    SchemaElement, TokenDatatype, UUIDDatatype,
};

use self::{
    associated_activity::AssociatedActivity, event_timing::EventTiming,
    task_dependency::TaskDependency,
};

pub mod associated_activity;
pub mod event_timing;
pub mod task_dependency;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Task {
    pub uuid: UUIDDatatype,
    /// "enum": [
    ///    "milestone",
    ///    "action"
    /// ]
    #[serde(rename = "type")]
    pub _type: TokenDatatype,
    pub title: String,
    pub description: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    pub timing: Option<EventTiming>,
    pub dependencies: Option<Vec<TaskDependency>>,
    pub tasks: Option<Vec<Task>>,
    pub associated_activities: Option<Vec<AssociatedActivity>>,
    /// "#assembly_oscal-assessment-common_assessment-subject"
    pub subjects: Option<Vec<super::assessment_subject::AssessmentSubject>>,
    /// "#assembly_oscal-metadata_responsible-role"
    pub responsible_roles: Option<Vec<responsible_role::ResponsibleRole>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaElement for Task {
    fn schema_title() -> &'static str {
        "Task"
    }
    fn schema_description() -> &'static str {
        r#"Represents a scheduled event or milestone, which may be associated with a series of assessment actions."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-assessment-common_task")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task"
    }
}
