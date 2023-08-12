/// Service Protocol Information
/// Information about the protocol used to provide a service.
/// $id: #assembly_oscal-implementation-common_protocol
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::PortRange;
use crate::definitions::StringDatatype;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Protocol {
	/// Service Protocol Information Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this service protocol information elsewhere in this or other OSCAL instances. The locally defined UUID of the service protocol can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: Option<UuidDatatype>,
	pub port_ranges: Option<Vec<PortRange>>,
	/// Protocol Title
	/// A human readable name for the protocol (e.g., Transport Layer Security).
	pub title: Option<String>,
	/// Protocol Name
	/// The common name of the protocol, which should be the appropriate "service name" from the IANA Service Name and Transport Protocol Port Number Registry.
	pub name: StringDatatype,
}
