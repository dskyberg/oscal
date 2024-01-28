/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_security_plan.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemSecurityPlan {
}

impl SchemaConstraint for SystemSecurityPlan {
    fn constraint_title() -> &'static str {
        "System Security Plan (SSP)"
    }
    fn constraint_description() -> &'static str {
        r#"A system security plan, such as those described in NIST SP 800-18"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_system-security-plan"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-security-plan"
    }
}
