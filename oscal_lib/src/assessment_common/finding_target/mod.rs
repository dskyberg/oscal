/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/finding_target.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::implementation_status::ImplementationStatus,
    metadata::{Link, Property, Remarks},
    SchemaElement, TokenDatatype,
};

use self::objective_status::ObjectiveStatus;

pub mod objective_status;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FindingTarget {
    #[serde(rename = "type")]
    pub _type: TokenDatatype,
    pub target_id: TokenDatatype,
    pub title: Option<String>,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub status: ObjectiveStatus,
    pub implementation_status: Option<ImplementationStatus>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for FindingTarget {
    fn schema_title() -> &'static str {
        "Objective Status"
    }
    fn schema_description() -> &'static str {
        r#"Captures an assessor's conclusions regarding the degree to which an objective is satisfied."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:finding-target"
    }
}
