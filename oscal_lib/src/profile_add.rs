use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    CommonParameter, CommonPart, MetadataLink, MetadataProperty, SchemaConstraint, TokenDatatype,
};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileAdd {
    pub position: Option<TokenDatatype>,
    pub by_id: Option<TokenDatatype>,
    pub title: Option<String>,
    pub params: Option<Vec<CommonParameter>>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub parts: Option<Vec<CommonPart>>,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "add[@position"}
    ],
    "allowed-values": [
        "before",
        "after",
        "starting",
        "ending"

    ]
}
 */

impl SchemaConstraint for ProfileAdd {
    fn constraint_title() -> &'static str {
        "Addition"
    }
    fn constraint_description() -> &'static str {
        "Specifies contents to be added into controls, in resolution"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_add"
    }
    fn schema_path() -> &'static str {
        "add"
    }
}
