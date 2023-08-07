pub use hash_algorithm::*;


pub mod hash_algorithm;

/// Hash
/// A representation of a cryptographic digest generated over a resource using a specified hash algorithm.
/// $id: #field_oscal-metadata_hash
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Hash {
	/// Hash algorithm
	/// Method by which a hash is derived
	pub algorithm: HashAlgorithm,
	pub value: StringDatatype,
}
