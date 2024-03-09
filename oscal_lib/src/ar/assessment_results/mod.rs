/// File name: ../oscal_lib/src/oscal_complete_oscal_ar/assessment_results.rs
/// pub use oscal_complete_oscal_ar::*;
///
/// pub mod oscal_complete_oscal_ar;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{BackMatter, Metadata},
    SchemaElement, UUIDDatatype,
};

use super::{import_ap::ImportAp, result::Result};

use self::local_definitions::LocalDefinitions;

pub mod local_definitions;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentResults {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub import_ap: ImportAp,
    pub local_definitions: Option<LocalDefinitions>,
    pub results: Vec<Result>,
    pub back_matter: Option<BackMatter>,
}

impl SchemaElement for AssessmentResults {
    fn schema_title() -> &'static str {
        "Security Assessment Results (SAR)"
    }
    fn schema_description() -> &'static str {
        r#"Security assessment results, such as those provided by a FedRAMP assessor in the FedRAMP Security Assessment Report."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ar_assessment-results")
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ar:assessment-results"
    }
}
