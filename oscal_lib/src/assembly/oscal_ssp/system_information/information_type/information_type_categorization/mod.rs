pub use information_type_identification_system::*;


pub mod information_type_identification_system;

/// Information Type Categorization
/// A set of information type identifiers qualified by the given identification system used, such as NIST SP 800-60.
/// $id: #assembly_oscal-ssp_system-information_information-type_information-type_information-type-categorization_information-type-categorization
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct InformationTypeCategorization {
	/// Information Type Identification System
	/// Specifies the information type identification system used.
	pub system: InformationTypeIdentificationSystem,
	pub information_type_ids: Option<Vec<StringDatatype>>,
}
