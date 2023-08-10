/// Set Parameter Value
/// Identifies the parameter that will be set by the enclosed value.
/// $id: #assembly_oscal-implementation-common_set-parameter
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;
use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SetParameter {
	pub remarks: Option<Remarks>,
	pub values: Vec<StringDatatype>,
	/// Parameter ID
	/// A human-oriented reference to a parameter within a control, who's catalog has been imported into the current implementation context.
	pub param_id: TokenDatatype,
}
