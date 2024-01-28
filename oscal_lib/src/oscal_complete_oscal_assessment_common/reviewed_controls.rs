/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/reviewed_controls.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_catalog_common::include_all,
    oscal_complete_oscal_metadata::{link, property, remarks},
    SchemaConstraint,
};

use super::{select_control_by_id, select_objective_by_id};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessedControls {
    pub description: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-catalog-common_include-all"
    pub include_all: Option<Vec<include_all::IncludeAll>>,
    /// "#assembly_oscal-assessment-common_select-control-by-id"
    pub include_controls: Option<Vec<select_control_by_id::SelectControlById>>,
    // "#assembly_oscal-assessment-common_select-control-by-id"
    pub exclude_controls: Option<Vec<select_control_by_id::SelectControlById>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for AssessedControls {
    fn constraint_title() -> &'static str {
        "Reviewed Controls and Control Objectives"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies the controls being assessed and their control objectives."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_reviewed-controls:assessed-controls"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:reviewed-controls:assessed-controls"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlObjectiveDescription {
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-catalog-common_include-all"
    pub include_all: Option<include_all::IncludeAll>,
    // "#assembly_oscal-assessment-common_select-objective-by-id"
    pub include_objectives: Option<Vec<select_objective_by_id::SelectObjectiveById>>,
    // "#assembly_oscal-assessment-common_select-objective-by-id"
    pub exclude_objectives: Option<Vec<select_objective_by_id::SelectObjectiveById>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for ControlObjectiveDescription {
    fn constraint_title() -> &'static str {
        "Reviewed Controls and Control Objectives"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies the controls being assessed and their control objectives."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_reviewed-controls:control-objective-description"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:reviewed-controls:control-objective-description"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewedControls {
    pub description: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    pub control_selections: Vec<AssessedControls>,
    pub control_objective_selections: Option<Vec<ControlObjectiveDescription>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
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
