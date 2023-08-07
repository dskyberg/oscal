/// Threat Type Identification System
/// Specifies the source of the threat information.
/// $id: #field_oscal-assessment-common_threat-id_threat-type-identification-system_threat-type-identification-system
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ThreatTypeIdentificationSystem {
	// orig: http://fedramp.gov
#[serde(rename = "http://fedramp.gov")]
	HttpFedrampGov,
	// orig: http://fedramp.gov/ns/oscal
#[serde(rename = "http://fedramp.gov/ns/oscal")]
	HttpFedrampGovNsOscal,
}
