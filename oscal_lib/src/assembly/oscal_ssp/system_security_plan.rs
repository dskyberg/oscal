/// System Security Plan (SSP)
/// A system security plan, such as those described in NIST SP 800-18
/// $id: #assembly_oscal-ssp_system-security-plan
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::BackMatter;
use crate::assembly::oscal_metadata::Metadata;
use crate::assembly::oscal_ssp::ControlImplementation;
use crate::assembly::oscal_ssp::ImportProfile;
use crate::assembly::oscal_ssp::SystemCharacteristics;
use crate::assembly::oscal_ssp::SystemImplementation;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SystemSecurityPlan {
	pub control_implementation: ControlImplementation,
	/// System Security Plan Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this system security plan (SSP) elsewhere in this or other OSCAL instances. The locally defined UUID of the SSP can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance).This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub back_matter: Option<BackMatter>,
	pub system_implementation: SystemImplementation,
	pub import_profile: ImportProfile,
	pub metadata: Metadata,
	pub system_characteristics: SystemCharacteristics,
}
