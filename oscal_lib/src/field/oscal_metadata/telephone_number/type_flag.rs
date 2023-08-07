/// type flag
/// Indicates the type of phone number.
/// $id: #field_oscal-metadata_telephone-number_type-flag_type-flag
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum TypeFlag {
	// orig: home
	Home,
	// orig: office
	Office,
	// orig: mobile
	Mobile,
}
