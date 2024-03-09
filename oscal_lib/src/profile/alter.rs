/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/alter.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, TokenDatatype};

use super::{add::Add, remove::Remove};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Alter {
    pub control_id: Option<TokenDatatype>,
    pub removes: Option<Vec<Remove>>,
    pub adds: Option<Vec<Add>>,
}

impl SchemaElement for Alter {
    fn schema_title() -> &'static str {
        "Alteration"
    }
    fn schema_description() -> &'static str {
        r#"An Alter element specifies changes to be made to an included control when a profile is resolved."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_alter")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:alter"
    }
}
