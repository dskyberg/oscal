use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::select_control_by_id::SelectControlById,
    catalog_common::include_all::IncludeAll,
    metadata::{Link, Property, Remarks},
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessedControl {
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub include_all: Option<IncludeAll>,
    pub include_controls: Option<Vec<SelectControlById>>,
    pub exclude_controls: Option<Vec<SelectControlById>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for AssessedControl {
    fn schema_title() -> &'static str {
        "Reviewed Controls and Control Objectives"
    }
    fn schema_description() -> &'static str {
        r#"Identifies the controls being assessed and their control objectives."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:reviewed-controls:assessed-controls"
    }
}
