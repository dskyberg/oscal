/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/security_impact_level.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SecurityImpactLevel {
}

impl SchemaConstraint for SecurityImpactLevel {
    fn constraint_title() -> &'static str {
        "Security Impact Level"
    }
    fn constraint_description() -> &'static str {
        r#"The overall level of expected impact resulting from unauthorized disclosure, modification, or loss of access to information."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_security-impact-level"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:security-impact-level"
    }
}
