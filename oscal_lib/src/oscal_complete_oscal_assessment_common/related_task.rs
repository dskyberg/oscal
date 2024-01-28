/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/related_task.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks, responsible_party},
    SchemaConstraint, UUIDDatatype,
};

use super::assessment_subject;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IdentifiedSubject {
    pub subject_placeholder_uuid: UUIDDatatype,
    /// "#assembly_oscal-assessment-common_assessment-subject"
    pub subjects: Vec<super::assessment_subject::AssessmentSubject>,
}

impl SchemaConstraint for IdentifiedSubject {
    fn constraint_title() -> &'static str {
        "Identified Subject"
    }
    fn constraint_description() -> &'static str {
        "Used to detail assessment subjects that were identfied by this task."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_related-task:identified-subject"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:related-task:identified-subject"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedTask {
    pub task_uuid: UUIDDatatype,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-metadata_responsible-party"
    pub responsible_parties: Option<Vec<responsible_party::ResponsibleParty>>,
    /// "#assembly_oscal-assessment-common_assessment-subject"
    pub subjects: Option<Vec<assessment_subject::AssessmentSubject>>,
    pub identified_subject: Option<IdentifiedSubject>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for RelatedTask {
    fn constraint_title() -> &'static str {
        "Task Reference"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies an individual task for which the containing object is a consequence of."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_related-task"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:related-task"
    }
}
