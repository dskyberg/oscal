/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_part.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property},
    SchemaConstraint, TokenDatatype, URIDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPart {
    pub uuid: Option<UUIDDatatype>,
    /// "enum": [
    ///    "asset",
    ///    "method",
    ///    "objective"
    /// ]
    pub name: TokenDatatype,
    pub ns: Option<URIDatatype>,
    pub class: Option<TokenDatatype>,
    pub title: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    pub prose: Option<String>,
    // "#assembly_oscal-assessment-common_assessment-part"
    pub parts: Option<Vec<AssessmentPart>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
}

impl SchemaConstraint for AssessmentPart {
    fn constraint_title() -> &'static str {
        "Assessment Part"
    }
    fn constraint_description() -> &'static str {
        r#"A partition of an assessment plan or results or a child of another part."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-part"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-part"
    }
}
