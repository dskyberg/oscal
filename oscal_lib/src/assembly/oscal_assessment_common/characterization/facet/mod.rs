pub use naming_system::*;


pub mod naming_system;

/// Facet
/// An individual characteristic that is part of a larger set produced by the same actor.
/// $id: #assembly_oscal-assessment-common_characterization_facet_facet
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::StringDatatype;
use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Facet {
	pub links: Option<Vec<Link>>,
	/// Facet Name
	/// The name of the risk metric within the specified system.
	pub name: TokenDatatype,
	pub remarks: Option<Remarks>,
	/// Naming System
	/// Specifies the naming system under which this risk metric is organized, which allows for the same names to be used in different systems controlled by different parties. This avoids the potential of a name clash.
	pub system: NamingSystem,
	/// Facet Value
	/// Indicates the value of the facet.
	pub value: StringDatatype,
	pub props: Option<Vec<Property>>,
}
