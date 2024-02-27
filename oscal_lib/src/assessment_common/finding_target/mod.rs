/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/finding_target.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::implementation_status::ImplementationStatus,
    metadata::{Link, Property, Remarks},
    SchemaConstraint, TokenDatatype,
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

impl SchemaConstraint for FindingTarget {
    fn constraint_title() -> &'static str {
        "Objective Status"
    }
    fn constraint_description() -> &'static str {
        r#"Captures an assessor's conclusions regarding the degree to which an objective is satisfied."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_finding-target"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:finding-target"
    }
}
