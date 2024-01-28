/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/finding_target.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_implementation_common::implementation_status,
    oscal_complete_oscal_metadata::{link, property, remarks},
    SchemaConstraint, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ObjectiveStatus {
    /// "enum": [
    ///    "satisfied",
    ///    "not-satisfied"
    /// ]
    pub state: TokenDatatype,
    /// "enum": [
    ///    "pass",
    ///    "fail",
    ///    "other"
    /// ]
    pub reason: Option<TokenDatatype>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for ObjectiveStatus {
    fn constraint_title() -> &'static str {
        "Objective Status"
    }
    fn constraint_description() -> &'static str {
        "A determination of if the objective is satisfied or not within a given system."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_finding-target:status"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:finding-target:status"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FindingTarget {
    #[serde(rename = "type")]
    pub _type: TokenDatatype,
    pub target_id: TokenDatatype,
    pub title: Option<String>,
    pub description: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    pub status: ObjectiveStatus,
    // "#assembly_oscal-implementation-common_implementation-status"
    pub implementation_status: Option<implementation_status::ImplementationStatus>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
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
