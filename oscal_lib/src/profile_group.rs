use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    CommonParameter, CommonPart, MetadataLink, MetadataProperty, ProfileInsertControl,
    SchemaConstraint, TokenDatatype,
};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileGroup {
    pub id: Option<TokenDatatype>,
    pub class: Option<TokenDatatype>,
    pub title: String,
    pub params: Option<Vec<CommonParameter>>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub parts: Option<Vec<CommonPart>>,
    pub groups: Option<Vec<ProfileGroup>>,
    pub insert_controls: Option<Vec<ProfileInsertControl>>,
}

impl SchemaConstraint for ProfileGroup {
    fn constraint_title() -> &'static str {
        "Control group"
    }
    fn constraint_description() -> &'static str {
        "A group of (selected) controls or of groups of controls"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_group"
    }
    fn schema_path() -> &'static str {
        "group"
    }
}
