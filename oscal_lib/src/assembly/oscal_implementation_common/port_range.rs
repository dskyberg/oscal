/// Port Range
/// Where applicable this is the IPv4 port range on which the service operates.
/// $id: #assembly_oscal-implementation-common_port-range
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::NonNegativeIntegerDatatype;
use crate::definitions::TokenDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct PortRange {
	/// End
	/// Indicates the ending port number in a port range
	pub end: Option<NonNegativeIntegerDatatype>,
	/// Transport
	/// Indicates the transport type.
	pub transport: Option<TokenDatatype>,
	/// Start
	/// Indicates the starting port number in a port range
	pub start: Option<NonNegativeIntegerDatatype>,
}
