use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    MetadataLink, MetadataProperty, MetadataRemarks, ParameterConstraint, ParameterGuideline,
    ParameterValue, SchemaConstraint, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CommonParameter {
    pub id: TokenDatatype,
    pub class: Option<TokenDatatype>,
    pub depends_on: Option<TokenDatatype>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub label: Option<String>,
    pub usage: Option<String>,
    pub constraints: Option<Vec<ParameterConstraint>>,
    pub guidelines: Option<Vec<ParameterGuideline>>,
    pub values: Option<Vec<ParameterValue>>,
    pub remars: Option<MetadataRemarks>,
}

impl SchemaConstraint for CommonParameter {
    fn constraint_title() -> &'static str {
        "Parameter"
    }
    fn constraint_description() -> &'static str {
        "Parameters provide a mechanism for the dynamic assignment of value(s) in a control."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter"
    }
    fn schema_path() -> &'static str {
        "parameter"
    }
}
