/// System Characteristics
/// Contains the characteristics of the system, such as its name, purpose, and security impact level.
/// $id: #assembly_oscal-ssp_system-characteristics
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleParty;
use crate::assembly::oscal_ssp::AuthorizationBoundary;
use crate::assembly::oscal_ssp::DataFlow;
use crate::assembly::oscal_ssp::NetworkArchitecture;
use crate::assembly::oscal_ssp::SecurityImpactLevel;
use crate::assembly::oscal_ssp::Status;
use crate::assembly::oscal_ssp::SystemInformation;
use crate::definitions::StringDatatype;
use crate::field::oscal_implementation_common::SystemId;
use crate::field::oscal_metadata::Remarks;
use crate::field::oscal_ssp::DateAuthorized;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SystemCharacteristics {
	pub data_flow: Option<DataFlow>,
	/// Security Sensitivity Level
	/// The overall information system sensitivity categorization, such as defined by FIPS-199.
	pub security_sensitivity_level: StringDatatype,
	pub date_authorized: Option<DateAuthorized>,
	pub system_information: SystemInformation,
	pub network_architecture: Option<NetworkArchitecture>,
	pub responsible_parties: Option<Vec<ResponsibleParty>>,
	/// System Name - Full
	/// The full name of the system.
	pub system_name: StringDatatype,
	pub props: Option<Vec<Property>>,
	pub system_ids: Vec<SystemId>,
	pub security_impact_level: SecurityImpactLevel,
	pub links: Option<Vec<Link>>,
	pub status: Status,
	pub authorization_boundary: AuthorizationBoundary,
	/// System Description
	/// A summary of the system.
	pub description: String,
	pub remarks: Option<Remarks>,
	/// System Name - Short
	/// A short name for the system, such as an acronym, that is suitable for display in a data table or summary list.
	pub system_name_short: Option<StringDatatype>,
}
