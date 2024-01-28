/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/subject_reference.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks},
    SchemaConstraint, TokenDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SubjectReference {
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
    pub title: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for SubjectReference {
    fn constraint_title() -> &'static str {
        "Identifies the Subject"
    }
    fn constraint_description() -> &'static str {
        r#"A human-oriented identifier reference to a resource. Use type to indicate whether the identified resource is a component, inventory item, location, user, or something else."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_subject-reference"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:subject-reference"
    }
}
