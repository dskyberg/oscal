/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/reviewed_controls.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint,
};

use self::{
    assessed_control::AssessedControl, control_objective_description::ControlObjectiveDescription,
};

pub mod assessed_control;
pub mod control_objective_description;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewedControls {
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub control_selections: Vec<AssessedControl>,
    pub control_objective_selections: Option<Vec<ControlObjectiveDescription>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for ReviewedControls {
    fn constraint_title() -> &'static str {
        "Reviewed Controls and Control Objectives"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies the controls being assessed and their control objectives."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_reviewed-controls"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:reviewed-controls"
    }
}
