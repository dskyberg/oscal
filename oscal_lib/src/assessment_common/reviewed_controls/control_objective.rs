use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::select_objective_by_id::SelectObjectiveById,
    catalog_common::include_all::IncludeAll,
    metadata::{Link, Property, Remarks},
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlObjective {
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub include_all: Option<IncludeAll>,
    pub include_objectives: Option<Vec<SelectObjectiveById>>,
    pub exclude_objectives: Option<Vec<SelectObjectiveById>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ControlObjective {
    fn schema_title() -> &'static str {
        "Referenced Control Objectives"
    }
    fn schema_description() -> &'static str {
        "Identifies the control objectives of the assessment. In the assessment plan, these are the planned objectives. In the assessment results, these are the assessed objectives, and reflects any changes from the plan."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:reviewed-controls:control-objective"
    }
}
