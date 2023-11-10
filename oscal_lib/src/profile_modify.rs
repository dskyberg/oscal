use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    MetadataLink, MetadataProperty, ParameterConstraint, ParameterGuideline, ParameterSelection,
    ParameterValue, SchemaConstraint, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterSetting {
    pub param_id: TokenDatatype,
    pub class: Option<TokenDatatype>,
    pub depends_on: Option<TokenDatatype>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub label: Option<String>,
    pub usage: Option<String>,
    pub constraints: Option<Vec<ParameterConstraint>>,
    pub guidelines: Option<Vec<ParameterGuideline>>,
    pub values: Option<Vec<ParameterValue>>,
    pub select: Option<ParameterSelection>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileModify {
    pub set_parameters: Vec<ParameterSetting>,
}

impl SchemaConstraint for ProfileModify {
    fn constraint_title() -> &'static str {
        "Modify controls"
    }
    fn constraint_description() -> &'static str {
        "Set parameters or amend controls in resolution"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_modify"
    }
    fn schema_path() -> &'static str {
        "modify"
    }
}
