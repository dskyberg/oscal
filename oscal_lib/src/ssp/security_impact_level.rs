/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/security_impact_level.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SecurityImpactLevel {
    pub security_objective_confidentiality: StringDatatype,
    pub security_objective_integrity: StringDatatype,
    pub security_objective_availability: StringDatatype,
}

impl SchemaElement for SecurityImpactLevel {
    fn schema_title() -> &'static str {
        "Security Impact Level"
    }
    fn schema_description() -> &'static str {
        r#"The overall level of expected impact resulting from unauthorized disclosure, modification, or loss of access to information."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_security-impact-level")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:security-impact-level"
    }
}
