/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/task.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{link, property, remarks, responsible_role},
    DateTimeWithTimezoneDatatype, PositiveIntegerDatatype, SchemaConstraint, StringDatatype,
    TokenDatatype, UUIDDatatype,
};

// TODO: Decompose

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnDateCondition {
    pub date: DateTimeWithTimezoneDatatype,
}

impl SchemaConstraint for OnDateCondition {
    fn constraint_title() -> &'static str {
        "On Date Condition"
    }
    fn constraint_description() -> &'static str {
        "The task is intended to occur on the specified date."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing:on-date-condition"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing:on-date-condition"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnDateRangeCondition {
    pub start: DateTimeWithTimezoneDatatype,
    pub end: DateTimeWithTimezoneDatatype,
}

impl SchemaConstraint for OnDateRangeCondition {
    fn constraint_title() -> &'static str {
        "On Date Range Condition"
    }
    fn constraint_description() -> &'static str {
        "The task is intended to occur within the specified date range."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing:on-date-condition-range"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing:on-date-condition-range"
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FrequencyCondition {
    pub period: PositiveIntegerDatatype,
    /// "enum": [
    ///    "seconds",
    ///    "minutes",
    ///    "hours",
    ///    "days",
    ///    "months",
    ///   "years"
    /// ]
    pub unit: StringDatatype,
}

impl SchemaConstraint for FrequencyCondition {
    fn constraint_title() -> &'static str {
        "Frequency Condition"
    }
    fn constraint_description() -> &'static str {
        "The task is intended to occur at the specified frequency."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventTiming {
    pub on_date: Option<OnDateCondition>,
    pub within_date_range: Option<OnDateRangeCondition>,
    pub at_frequency: Option<FrequencyCondition>,
}

impl SchemaConstraint for EventTiming {
    fn constraint_title() -> &'static str {
        "Event Timing"
    }
    fn constraint_description() -> &'static str {
        "The timing under which the task is intended to occur."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaskDependency {
    pub task_uuid: UUIDDatatype,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
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

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssociatedActivity {
    pub activity_uuid: UUIDDatatype,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-metadata_responsible-role"
    pub responsible_roles: Option<Vec<responsible_role::ResponsibleRole>>,
    /// TODO: "#assembly_oscal-assessment-common_assessment-subject"
    pub subjects: Vec<super::assessment_subject::AssessmentSubject>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for AssociatedActivity {
    fn constraint_title() -> &'static str {
        "Associated Activity"
    }
    fn constraint_description() -> &'static str {
        "Identifies an individual activity to be performed as part of a task."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:associated-activity"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:associated-activity"
    }
}

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

impl SchemaConstraint for Task {
    fn constraint_title() -> &'static str {
        "Task"
    }
    fn constraint_description() -> &'static str {
        r#"Represents a scheduled event or milestone, which may be associated with a series of assessment actions."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task"
    }
}
