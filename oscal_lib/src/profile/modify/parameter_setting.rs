use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    catalog_common::{
        parameter_constraint::ParameterConstraint, parameter_guideline::ParameterGuideline,
        parameter_selection::ParameterSelection, parameter_value::ParameterValue,
    },
    metadata::{Link, Property},
    SchemaElement, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterSetting {
    pub param_id: TokenDatatype,
    pub class: Option<TokenDatatype>,
    pub depends_on: Option<TokenDatatype>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub label: Option<String>,
    pub usage: Option<String>,
    pub constraints: Option<Vec<ParameterConstraint>>,
    pub guidelines: Option<Vec<ParameterGuideline>>,
    pub values: Option<Vec<ParameterValue>>,
    pub select: Option<ParameterSelection>,
}

impl SchemaElement for ParameterSetting {
    fn schema_title() -> &'static str {
        "Parameter Setting"
    }
    fn schema_description() -> &'static str {
        "A parameter setting, to be propagated to points of insertion"
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:modify:set_parameter"
    }
}
