/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/add.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
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
pub struct Add {
    /// "enum": [
    ///     "before",
    ///     "after",
    ///     "starting",
    ///     "ending"
    /// ]
    pub position: Option<TokenDatatype>,
    pub by_id: Option<TokenDatatype>,
    pub title: Option<String>,
    pub params: Option<Vec<Parameter>>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub parts: Option<Vec<Part>>,
}

impl SchemaElement for Add {
    fn schema_title() -> &'static str {
        "Addition"
    }
    fn schema_description() -> &'static str {
        r#"Specifies contents to be added into controls, in resolution"#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_add")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:add"
    }
}
