/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_subject.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_catalog_common::include_all,
    oscal_complete_oscal_metadata::{link, property, remarks},
    SchemaConstraint, TokenDatatype,
};

use super::select_subject_by_id;

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
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    //. "#assembly_oscal-catalog-common_include-all"
    pub include_all: Option<include_all::IncludeAll>,
    /// "#assembly_oscal-assessment-common_select-subject-by-id"
    pub include_subjects: Option<Vec<select_subject_by_id::SelectSubjectById>>,
    // "#assembly_oscal-assessment-common_select-subject-by-id"
    pub exclude_subjects: Option<Vec<select_subject_by_id::SelectSubjectById>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for AssessmentSubject {
    fn constraint_title() -> &'static str {
        "Subject of Assessment"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies system elements being assessed, such as components, inventory items, and locations. In the assessment plan, this identifies a planned assessment subject. In the assessment results this is an actual assessment subject, and reflects any changes from the plan. exactly what will be the focus of this assessment. Any subjects not identified in this way are out-of-scope."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-subject"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-subject"
    }
}
