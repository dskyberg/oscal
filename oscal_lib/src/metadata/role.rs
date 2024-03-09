/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/role.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype, TokenDatatype};

use super::{Link, Property, Remarks};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Role {
    pub id: TokenDatatype,
    pub title: String,
    pub short_name: Option<StringDatatype>,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Role {
    fn schema_title() -> &'static str {
        "Role"
    }
    fn schema_description() -> &'static str {
        r#"Defines a function assumed or expected to be assumed by a party in a specific situation."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_role")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:role"
    }
}
