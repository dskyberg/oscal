/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog/control.rs
/// pub use oscal_complete_oscal_catalog::*;
///
/// pub mod oscal_complete_oscal_catalog;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    catalog_common::{parameter::Parameter, part::Part},
    metadata::{Link, Property},
    SchemaElement, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Control {
    pub id: TokenDatatype,
    pub class: Option<TokenDatatype>,
    pub title: String,
    pub params: Option<Vec<Parameter>>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub parts: Option<Vec<Part>>,
    pub controls: Option<Vec<Control>>,
}

impl SchemaElement for Control {
    fn schema_title() -> &'static str {
        "Control"
    }
    fn schema_description() -> &'static str {
        r#"A structured information object representing a security or privacy control. Each security or privacy control within the Catalog is defined by a distinct control instance."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-catalog_control")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog:control"
    }
}
