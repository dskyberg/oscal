/// Identification System Type
/// Identifies the identification system from which the provided identifier was assigned.
/// $id: #field_oscal-implementation-common_system-id_identification-system-type_identification-system-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum IdentificationSystemType {
	// orig: https://fedramp.gov
#[serde(rename = "https://fedramp.gov")]
	HttpsFedrampGov,
	// orig: http://fedramp.gov/ns/oscal
#[serde(rename = "http://fedramp.gov/ns/oscal")]
	HttpFedrampGovNsOscal,
	// orig: https://ietf.org/rfc/rfc4122
#[serde(rename = "https://ietf.org/rfc/rfc4122")]
	HttpsIetfOrgRfcRfc4122,
	// orig: http://ietf.org/rfc/rfc4122
#[serde(rename = "http://ietf.org/rfc/rfc4122")]
	HttpIetfOrgRfcRfc4122,
}
