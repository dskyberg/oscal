use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::assessment_subject::AssessmentSubject, SchemaElement, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IdentifiedSubject {
    pub subject_placeholder_uuid: UUIDDatatype,
    pub subjects: Vec<AssessmentSubject>,
}

impl SchemaElement for IdentifiedSubject {
    fn schema_title() -> &'static str {
        "Identified Subject"
    }
    fn schema_description() -> &'static str {
        "Used to detail assessment subjects that were identfied by this task."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:related-task:identified-subject"
    }
}
