pub use threat_type_identification_system::*;


pub mod threat_type_identification_system;

/// Threat ID
/// A pointer, by ID, to an externally-defined threat.
/// $id: #field_oscal-assessment-common_threat-id
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UriDatatype;
use crate::definitions::UriReferenceDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ThreatId {
	/// Threat Type Identification System
	/// Specifies the source of the threat information.
	pub system: ThreatTypeIdentificationSystem,
	/// Threat Information Resource Reference
	/// An optional location for the threat data, from which this ID originates.
	pub href: Option<UriReferenceDatatype>,
	pub id: UriDatatype,
}
