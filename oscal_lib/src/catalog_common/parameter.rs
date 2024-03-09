/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/parameter.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaElement, TokenDatatype,
};

use super::{
    parameter_constraint::ParameterConstraint, parameter_guideline::ParameterGuideline,
    parameter_selection::ParameterSelection, parameter_value::ParameterValue,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Parameter {
    pub id: TokenDatatype,
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
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Parameter {
    fn schema_title() -> &'static str {
        "Parameter"
    }
    fn schema_description() -> &'static str {
        r#"Parameters provide a mechanism for the dynamic assignment of value(s) in a control."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-catalog-common_parameter")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:parameter"
    }
}
