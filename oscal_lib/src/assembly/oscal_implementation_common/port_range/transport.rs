/// Transport
/// Indicates the transport type.
/// $id: #assembly_oscal-implementation-common_port-range_transport_transport
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum Transport {
	// orig: TCP
	#[serde(rename = "TCP")]
	Tcp,
	// orig: UDP
	#[serde(rename = "UDP")]
	Udp,
}
