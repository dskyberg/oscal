/// File name: ../oscal_lib/src/oscal_complete_oscal_ar/finding.rs
/// pub use oscal_complete_oscal_ar::*;
///
/// pub mod oscal_complete_oscal_ar;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Finding {
}

impl SchemaConstraint for Finding {
    fn constraint_title() -> &'static str {
        "Finding"
    }
    fn constraint_description() -> &'static str {
        r#"Describes an individual finding."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ar_finding"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:finding"
    }
}
