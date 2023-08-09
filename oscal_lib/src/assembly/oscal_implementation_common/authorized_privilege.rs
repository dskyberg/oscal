/// Privilege
/// Identifies a specific system privilege held by the user, along with an associated description and/or rationale for the privilege.
/// $id: #assembly_oscal-implementation-common_authorized-privilege
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::field::oscal_implementation_common::FunctionPerformed;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AuthorizedPrivilege {
	pub functions_performed: Vec<FunctionPerformed>,
	/// Privilege Description
	/// A summary of the privilege's purpose within the system.
	pub description: Option<String>,
	/// Privilege Title
	/// A human readable name for the privilege.
	pub title: String,
}
