use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{activity::Activity, local_objective::LocalObjective},
    metadata::Remarks,
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalDefinitions {
    pub objectives_and_methods: Option<Vec<LocalObjective>>,
    pub activities: Option<Vec<Activity>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for LocalDefinitions {
    fn schema_title() -> &'static str {
        "Local Definitions"
    }
    fn schema_description() -> &'static str {
        "Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ar:assessment-results/local-definitions"
    }
}
