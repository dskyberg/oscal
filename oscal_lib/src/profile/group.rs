/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/group.rs
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

use super::insert_controls::InsertControls;

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
    pub insert_controls: Option<Vec<InsertControls>>,
}

impl SchemaElement for Group {
    fn schema_title() -> &'static str {
        "Control group"
    }
    fn schema_description() -> &'static str {
        r#"A group of (selected) controls or of groups of controls"#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_group")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:group"
    }
}
