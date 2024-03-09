/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/local_objective.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    catalog_common::part::Part,
    metadata::{Link, Property},
    SchemaElement, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalObjective {
    pub control_id: TokenDatatype,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub parts: Vec<Part>,
}

impl SchemaElement for LocalObjective {
    fn schema_title() -> &'static str {
        "Assessment-Specific Control Objective"
    }
    fn schema_description() -> &'static str {
        r#"A local definition of a control objective for this assessment. Uses catalog syntax for control objective and assessment actions."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-assessment-common_local-objective")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:local-objective"
    }
}
