pub use actor_type::*;


pub mod actor_type;

/// Originating Actor
/// The actor that produces an observation, a finding, or a risk. One or more actor type can be used to specify a person that is using a tool.
/// $id: #assembly_oscal-assessment-common_origin-actor
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct OriginActor {
	/// Actor Role
	/// For a party, this can optionally be used to specify the role the actor was performing.
	pub role_id: Option<TokenDatatype>,
	/// Actor Type
	/// The kind of actor.
	#[serde(rename = "type")]
	pub _type: ActorType,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Actor Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to the tool or person based on the associated type.
	pub actor_uuid: UuidDatatype,
}
