pub use information_type::*;


pub mod information_type;

/// System Information
/// Contains details about all information types that are stored, processed, or transmitted by the system, such as privacy information, and those defined in NIST SP 800-60.
/// $id: #assembly_oscal-ssp_system-information
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SystemInformation {
	pub props: Option<Vec<Property>>,
	pub information_types: Vec<InformationType>,
	pub links: Option<Vec<Link>>,
}
