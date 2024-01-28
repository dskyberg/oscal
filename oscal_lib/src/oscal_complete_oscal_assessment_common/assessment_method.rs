/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_method.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentMethod {
    pub uuid: UUIDDatatype,
    pub description: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    // "#assembly_oscal-assessment-common_assessment-part"
    pub part: super::assessment_part::AssessmentPart,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for AssessmentMethod {
    fn constraint_title() -> &'static str {
        "Assessment Method"
    }
    fn constraint_description() -> &'static str {
        r#"A local definition of a control objective. Uses catalog syntax for control objective and assessment activities."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-method"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-method"
    }
}
