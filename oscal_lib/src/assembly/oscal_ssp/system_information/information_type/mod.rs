pub use availability_impact_level::*;
pub use information_type_categorization::*;
pub use confidentiality_impact_level::*;
pub use integrity_impact_level::*;


pub mod availability_impact_level;
pub mod information_type_categorization;
pub mod confidentiality_impact_level;
pub mod integrity_impact_level;

/// Information Type
/// Contains details about one information type that is stored, processed, or transmitted by the system, such as privacy information, and those defined in NIST SP 800-60.
/// $id: #assembly_oscal-ssp_system-information_information-type_information-type
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct InformationType {
	/// title field
	/// A human readable name for the information type. This title should be meaningful within the context of the system.
	pub title: String,
	/// Confidentiality Impact Level
	/// The expected level of impact resulting from the unauthorized disclosure of the described information.
	pub confidentiality_impact: ConfidentialityImpactLevel,
	/// Availability Impact Level
	/// The expected level of impact resulting from the disruption of access to or use of the described information or the information system.
	pub availability_impact: AvailabilityImpactLevel,
	pub props: Option<Vec<Property>>,
	/// Information Type Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this information type elsewhere in this or other OSCAL instances. The locally defined UUID of the information type can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: Option<UuidDatatype>,
	pub links: Option<Vec<Link>>,
	pub categorizations: Option<Vec<InformationTypeCategorization>>,
	/// Integrity Impact Level
	/// The expected level of impact resulting from the unauthorized modification of the described information.
	pub integrity_impact: IntegrityImpactLevel,
	/// Information Type Description
	/// A summary of how this information type is used within the system.
	pub description: String,
}
