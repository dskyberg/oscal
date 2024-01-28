/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_subject_placeholder.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubjectSource {
    pub task_uuid: UUIDDatatype,
}

impl SchemaConstraint for AssessmentSubjectSource {
    fn constraint_title() -> &'static str {
        "Assessment Subject Source"
    }
    fn constraint_description() -> &'static str {
        "Assessment subjects will be identified while conducting the referenced activity-instance."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-subject-placeholder:sources"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-subject-placeholder:sources"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubjectPlaceholder {
    pub uuid: UUIDDatatype,
    pub description: Option<String>,
    pub sources: Vec<AssessmentSubjectSource>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for AssessmentSubjectPlaceholder {
    fn constraint_title() -> &'static str {
        "Assessment Subject Placeholder"
    }
    fn constraint_description() -> &'static str {
        r#"Used when the assessment subjects will be determined as part of one or more other assessment activities. These assessment subjects will be recorded in the assessment results in the assessment log."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-subject-placeholder"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-subject-placeholder"
    }
}
