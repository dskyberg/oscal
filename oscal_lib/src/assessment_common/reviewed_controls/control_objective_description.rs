use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::select_objective_by_id::SelectObjectiveById,
    catalog_common::include_all::IncludeAll,
    metadata::{Link, Property, Remarks},
    SchemaConstraint,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlObjectiveDescription {
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub include_all: Option<IncludeAll>,
    pub include_objectives: Option<Vec<SelectObjectiveById>>,
    pub exclude_objectives: Option<Vec<SelectObjectiveById>>,
    pub remarks: Option<Remarks>,
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
