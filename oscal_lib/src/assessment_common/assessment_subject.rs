/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_subject.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    catalog_common::include_all::IncludeAll,
    metadata::{Link, Property, Remarks},
    SchemaElement, TokenDatatype,
};

use super::select_subject_by_id::SelectSubjectById;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubject {
    /// "enum": [
    ///    "component",
    ///    "inventory-item",
    ///    "location",
    ///    "party",
    ///    "user"
    /// ]
    #[serde(rename = "type")]
    pub _type: TokenDatatype,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub include_all: Option<IncludeAll>,
    pub include_subjects: Option<Vec<SelectSubjectById>>,
    pub exclude_subjects: Option<Vec<SelectSubjectById>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<Remarks>,
}

impl SchemaElement for AssessmentSubject {
    fn schema_title() -> &'static str {
        "Subject of Assessment"
    }
    fn schema_description() -> &'static str {
        r#"Identifies system elements being assessed, such as components, inventory items, and locations. In the assessment plan, this identifies a planned assessment subject. In the assessment results this is an actual assessment subject, and reflects any changes from the plan. exactly what will be the focus of this assessment. Any subjects not identified in this way are out-of-scope."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-assessment-common_assessment-subject")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-subject"
    }
}
