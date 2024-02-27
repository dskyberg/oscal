use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::subject_reference::SubjectReference,
    metadata::{Link, Property, Remarks},
    SchemaConstraint, UUIDDatatype,
};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RequiredAsset {
    pub uuid: UUIDDatatype,
    pub subjects: Option<Vec<SubjectReference>>,
    pub title: Option<String>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
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
