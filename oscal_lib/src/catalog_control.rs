use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    CommonParameter, CommonPart, MetadataLink, MetadataProperty, SchemaConstraint, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CatalogControl {
    pub id: TokenDatatype,
    pub class: Option<TokenDatatype>,
    pub title: String,
    pub params: Option<Vec<CommonParameter>>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub parts: Option<Vec<CommonPart>>,
    pub controls: Option<Vec<CatalogControl>>,
}

impl SchemaConstraint for CatalogControl {
    fn constraint_title() -> &'static str {
        "Control"
    }
    fn constraint_description() -> &'static str {
        "A structured information object representing a security or privacy control. Each security or privacy control within the Catalog is defined by a distinct control instance."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog_control"
    }
    fn schema_path() -> &'static str {
        "control"
    }
}
