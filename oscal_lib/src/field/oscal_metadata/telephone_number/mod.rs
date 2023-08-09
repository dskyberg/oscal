pub use type_flag::*;


pub mod type_flag;

/// Telephone Number
/// Contact number by telephone.
/// $id: #field_oscal-metadata_telephone-number
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct TelephoneNumber {
	pub number: StringDatatype,
	/// type flag
	/// Indicates the type of phone number.
	#[serde(rename = "type")]
	pub _type: Option<TypeFlag>,
}
