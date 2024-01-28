/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/response.rs
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
pub struct RequiredAsset {
    pub uuid: UUIDDatatype,
    // "#assembly_oscal-assessment-common_subject-reference"
    pub subjects: Option<Vec<super::subject_reference::SubjectReference>>,
    pub title: Option<String>,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for RequiredAsset {
    fn constraint_title() -> &'static str {
        "Required Asset"
    }
    fn constraint_description() -> &'static str {
        "Identifies an asset required to achieve remediation."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_response:required-asset"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:response:required-asset"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Response {
    pub uuid: UUIDDatatype,
    /// "enum": [
    ///    "recommendation",
    ///    "planned",
    ///    "completed"
    /// ]
    pub lifecycle: TokenDatatype,
    pub title: String,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-assessment-common_origin"
    pub origins: Option<Vec<super::origin::Origin>>,
    pub required_assets: Option<Vec<RequiredAsset>>,
    // "#assembly_oscal-assessment-common_task"
    pub tasks: Option<Vec<super::task::Task>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for Response {
    fn constraint_title() -> &'static str {
        "Risk Response"
    }
    fn constraint_description() -> &'static str {
        r#"Describes either recommended or an actual plan for addressing the risk."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_response"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:response"
    }
}
