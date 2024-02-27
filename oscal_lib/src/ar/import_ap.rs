/// File name: ../oscal_lib/src/oscal_complete_oscal_ar/import_ap.rs
/// pub use oscal_complete_oscal_ar::*;
///
/// pub mod oscal_complete_oscal_ar;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaConstraint, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportAp {
    pub href: URIReferenceDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for ImportAp {
    fn constraint_title() -> &'static str {
        "Import Assessment Plan"
    }
    fn constraint_description() -> &'static str {
        r#"Used by assessment-results to import information about the original plan for assessing the system."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ar_import-ap"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:import-ap"
    }
}
