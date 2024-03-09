/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/import.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{catalog_common::include_all::IncludeAll, SchemaElement, URIReferenceDatatype};

use super::select_control_by_id::SelectControlById;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Import {
    pub href: URIReferenceDatatype,
    pub include_all: Option<IncludeAll>,
    pub include_controls: Option<Vec<SelectControlById>>,
    pub exclude_controls: Option<Vec<SelectControlById>>,
}

impl SchemaElement for Import {
    fn schema_title() -> &'static str {
        "Import resource"
    }
    fn schema_description() -> &'static str {
        r#"The import designates a catalog or profile to be included (referenced and potentially modified) by this profile. The import also identifies which controls to select using the include-all, include-controls, and exclude-controls directives."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_import")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:import"
    }
}
