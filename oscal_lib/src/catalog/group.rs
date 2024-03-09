/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog/group.rs
/// pub use oscal_complete_oscal_catalog::*;
///
/// pub mod oscal_complete_oscal_catalog;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::control::Control;

use crate::{
    catalog_common::{parameter::Parameter, part::Part},
    metadata::{Link, Property},
    SchemaElement, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
    pub id: Option<TokenDatatype>,
    pub class: Option<TokenDatatype>,
    pub title: String,
    pub params: Option<Vec<Parameter>>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub parts: Option<Vec<Part>>,
    pub groups: Option<Vec<Group>>,
    pub controls: Option<Vec<Control>>,
}

impl SchemaElement for Group {
    fn schema_title() -> &'static str {
        "Control Group"
    }
    fn schema_description() -> &'static str {
        r#"A group of controls, or of groups of controls."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-catalog_group")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog:group"
    }
}
