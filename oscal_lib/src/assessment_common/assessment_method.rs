/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_method.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint, UUIDDatatype,
};

use super::assessment_part::AssessmentPart;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentMethod {
    pub uuid: UUIDDatatype,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub part: AssessmentPart,
    pub remarks: Option<Remarks>,
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
