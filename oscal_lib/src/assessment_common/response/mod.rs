/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/response.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaElement, TokenDatatype, UUIDDatatype,
};

use super::{origin::Origin, task::Task};

use self::required_asset::RequiredAsset;

pub mod required_asset;

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
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub origins: Option<Vec<Origin>>,
    pub required_assets: Option<Vec<RequiredAsset>>,
    pub tasks: Option<Vec<Task>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Response {
    fn schema_title() -> &'static str {
        "Risk Response"
    }
    fn schema_description() -> &'static str {
        r#"Describes either recommended or an actual plan for addressing the risk."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:response"
    }
}
