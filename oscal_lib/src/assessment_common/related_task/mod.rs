/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/related_task.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleParty},
    SchemaElement, UUIDDatatype,
};

use super::assessment_subject::AssessmentSubject;

use self::identified_subject::IdentifiedSubject;

pub mod identified_subject;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedTask {
    pub task_uuid: UUIDDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    pub subjects: Option<Vec<AssessmentSubject>>,
    pub identified_subject: Option<IdentifiedSubject>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for RelatedTask {
    fn schema_title() -> &'static str {
        "Task Reference"
    }
    fn schema_description() -> &'static str {
        r#"Identifies an individual task for which the containing object is a consequence of."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:related-task"
    }
}
