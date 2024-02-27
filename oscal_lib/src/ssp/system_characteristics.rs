/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_characteristics.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemCharacteristics {
}

impl SchemaConstraint for SystemCharacteristics {
    fn constraint_title() -> &'static str {
        "System Characteristics"
    }
    fn constraint_description() -> &'static str {
        r#"Contains the characteristics of the system, such as its name, purpose, and security impact level."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_system-characteristics"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-characteristics"
    }
}
