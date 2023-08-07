/// Information Type Identification System
/// Specifies the information type identification system used.
/// $id: #assembly_oscal-ssp_system-information_information-type_information-type_information-type-categorization_information-type-identification-system_information-type-identification-system
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum InformationTypeIdentificationSystem {
	// orig: http://doi.org/10.6028/NIST.SP.800-60v2r1
#[serde(rename = "http://doi.org/10.6028/NIST.SP.800-60v2r1")]
	HttpDoiOrg106028NistSp80060V2R1,
}
