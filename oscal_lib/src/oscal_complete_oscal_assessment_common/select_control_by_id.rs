/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/select_control_by_id.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectControlById {
    pub control_id: TokenDatatype,
    pub statement_ids: Option<Vec<TokenDatatype>>,
}

impl SchemaConstraint for SelectControlById {
    fn constraint_title() -> &'static str {
        "Select Control"
    }
    fn constraint_description() -> &'static str {
        r#"Used to select a control for inclusion/exclusion based on one or more control identifiers. A set of statement identifiers can be used to target the inclusion/exclusion to only specific control statements providing more granularity over the specific statements that are within the asessment scope."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_select-control-by-id"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:select-control-by-id"
    }
}
