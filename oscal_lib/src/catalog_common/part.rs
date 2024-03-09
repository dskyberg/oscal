/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/part.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    SchemaElement, TokenDatatype, URIDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Part {
    pub id: Option<TokenDatatype>,
    pub name: TokenDatatype,
    pub ns: Option<URIDatatype>,
    pub class: Option<TokenDatatype>,
    pub props: Option<Vec<Property>>,
    pub prose: Option<String>,
    pub parts: Option<Vec<Part>>,
    pub links: Option<Vec<Link>>,
}

impl SchemaElement for Part {
    fn schema_title() -> &'static str {
        "Part"
    }
    fn schema_description() -> &'static str {
        r#"A partition of a control's definition or a child of another part."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-catalog-common_part")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:part"
    }
}
