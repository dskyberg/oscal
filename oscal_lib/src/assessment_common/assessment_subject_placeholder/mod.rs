/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_subject_placeholder.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint, UUIDDatatype,
};

use self::assessment_subject_source::AssessmentSubjectSource;

pub mod assessment_subject_source;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubjectPlaceholder {
    pub uuid: UUIDDatatype,
    pub description: Option<String>,
    pub sources: Vec<AssessmentSubjectSource>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
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
