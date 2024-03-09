/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/select_objective_by_id.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectObjectiveById {
    pub objective_id: TokenDatatype,
}

impl SchemaElement for SelectObjectiveById {
    fn schema_title() -> &'static str {
        "Select Objective"
    }
    fn schema_description() -> &'static str {
        r#"Used to select a control objective for inclusion/exclusion based on the control objective's identifier."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-assessment-common_select-objective-by-id")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:select-objective-by-id"
    }
}
