/// Hash algorithm
/// Method by which a hash is derived
/// $id: #field_oscal-metadata_hash_hash-algorithm_hash-algorithm
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum HashAlgorithm {
	// orig: SHA-224
	Sha224,
	// orig: SHA-256
	Sha256,
	// orig: SHA-384
	Sha384,
	// orig: SHA-512
	Sha512,
	// orig: SHA3-224
	Sha3224,
	// orig: SHA3-256
	Sha3256,
	// orig: SHA3-384
	Sha3384,
	// orig: SHA3-512
	Sha3512,
}
