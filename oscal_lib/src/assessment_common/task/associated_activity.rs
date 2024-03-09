use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::assessment_subject::AssessmentSubject,
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaElement, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssociatedActivity {
    pub activity_uuid: UUIDDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub subjects: Vec<AssessmentSubject>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for AssociatedActivity {
    fn schema_title() -> &'static str {
        "Associated Activity"
    }
    fn schema_description() -> &'static str {
        "Identifies an individual activity to be performed as part of a task."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:associated-activity"
    }
}
