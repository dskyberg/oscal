/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/select_subject_by_id.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint, TokenDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectSubjectById {
    pub subject_uuid: UUIDDatatype,
    /// "enum": [
    ///    "component",
    ///    "inventory-item",
    ///    "location",
    ///    "party",
    ///    "user",
    ///    "resource"
    /// ]
    #[serde(rename = "type")]
    pub _type: TokenDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for SelectSubjectById {
    fn constraint_title() -> &'static str {
        "Select Assessment Subject"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies a set of assessment subjects to include/exclude by UUID."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_select-subject-by-id"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:select-subject-by-id"
    }
}
