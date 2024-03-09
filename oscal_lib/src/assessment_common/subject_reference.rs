/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/subject_reference.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaElement, TokenDatatype, UUIDDatatype,
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
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for SubjectReference {
    fn schema_title() -> &'static str {
        "Identifies the Subject"
    }
    fn schema_description() -> &'static str {
        r#"A human-oriented identifier reference to a resource. Use type to indicate whether the identified resource is a component, inventory item, location, user, or something else."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-assessment-common_subject-reference")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:subject-reference"
    }
}
