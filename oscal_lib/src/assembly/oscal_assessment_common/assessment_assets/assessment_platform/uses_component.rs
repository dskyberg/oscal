/// Uses Component
/// The set of components that are used by the assessment platform.
/// $id: #assembly_oscal-assessment-common_assessment-assets_assessment-platform_assessment-platform_uses-component_uses-component
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleParty;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct UsesComponent {
	pub links: Option<Vec<Link>>,
	pub responsible_parties: Option<Vec<ResponsibleParty>>,
	pub remarks: Option<Remarks>,
	/// Component Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a component that is implemented as part of an inventory item.
	pub component_uuid: UuidDatatype,
	pub props: Option<Vec<Property>>,
}
