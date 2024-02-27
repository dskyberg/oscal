/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/select_objective_by_id.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectObjectiveById {
    pub objective_id: TokenDatatype,
}

impl SchemaConstraint for SelectObjectiveById {
    fn constraint_title() -> &'static str {
        "Select Objective"
    }
    fn constraint_description() -> &'static str {
        r#"Used to select a control objective for inclusion/exclusion based on the control objective's identifier."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_select-objective-by-id"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:select-objective-by-id"
    }
}
