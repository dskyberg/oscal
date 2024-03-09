/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/import_profile.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaElement, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportProfile {
    pub href: URIReferenceDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ImportProfile {
    fn schema_title() -> &'static str {
        "Import Profile"
    }
    fn schema_description() -> &'static str {
        r#"Used to import the OSCAL profile representing the system's control baseline."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_import-profile")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:import-profile"
    }
}
