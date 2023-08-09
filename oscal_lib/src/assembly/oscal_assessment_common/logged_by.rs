/// Logged By
/// Used to indicate who created a log entry in what role.
/// $id: #assembly_oscal-assessment-common_logged-by
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::TokenDatatype;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct LoggedBy {
	/// Actor Role
	/// A point to the role-id of the role in which the party is making the log entry.
	pub role_id: Option<TokenDatatype>,
	/// Party UUID Reference
	/// A machine-oriented identifier reference to the party who is making the log entry.
	pub party_uuid: UuidDatatype,
}
