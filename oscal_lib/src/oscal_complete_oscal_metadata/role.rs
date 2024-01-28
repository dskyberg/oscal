/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/role.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Role {
    pub id: TokenDatatype,
    pub title: String,
    pub short_name: Option<StringDatatype>,
    pub description: Option<String>,
    /// #assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<super::remarks::Remarks>,
}

impl SchemaConstraint for Role {
    fn constraint_title() -> &'static str {
        "Role"
    }
    fn constraint_description() -> &'static str {
        r#"Defines a function assumed or expected to be assumed by a party in a specific situation."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_role"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:role"
    }
}
