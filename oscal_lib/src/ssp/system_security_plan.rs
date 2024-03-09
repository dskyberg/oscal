/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_security_plan.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{BackMatter, Metadata},
    SchemaElement, UUIDDatatype,
};

use super::{
    control_implementation::ControlImplementation, import_profile::ImportProfile,
    system_characteristics::SystemCharacteristics, system_implementation::SystemImplementation,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemSecurityPlan {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub import_profile: ImportProfile,
    pub system_characteristics: SystemCharacteristics,
    pub system_implementation: SystemImplementation,
    pub control_implementation: ControlImplementation,
    pub back_matter: Option<BackMatter>,
}

impl SchemaElement for SystemSecurityPlan {
    fn schema_title() -> &'static str {
        "System Security Plan (SSP)"
    }
    fn schema_description() -> &'static str {
        r#"A system security plan, such as those described in NIST SP 800-18"#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_system-security-plan")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-security-plan"
    }
}
