use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    CatalogControl, CommonParameter, CommonPart, MetadataLink, MetadataProperty, SchemaConstraint,
    TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CatalogGroup {
    pub id: Option<TokenDatatype>,
    pub class: Option<TokenDatatype>,
    pub title: String,
    pub params: Option<Vec<CommonParameter>>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub parts: Option<Vec<CommonPart>>,
    pub groups: Option<Vec<CatalogGroup>>,
    pub controls: Option<Vec<CatalogControl>>,
}

impl SchemaConstraint for CatalogGroup {
    fn constraint_title() -> &'static str {
        "Control Group"
    }

    fn constraint_description() -> &'static str {
        "A group of controls, or of groups of controls."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog_group"
    }

    fn schema_path() -> &'static str {
        "group"
    }
}
