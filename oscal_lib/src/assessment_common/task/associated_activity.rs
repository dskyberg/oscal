use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::assessment_subject::AssessmentSubject,
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaConstraint, UUIDDatatype,
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
