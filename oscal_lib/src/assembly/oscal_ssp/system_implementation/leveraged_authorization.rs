/// Leveraged Authorization
/// A description of another authorized system from which this system inherits capabilities that satisfy security requirements. Another term for this concept is a common control provider.
/// $id: #assembly_oscal-ssp_system-implementation_leveraged-authorization_leveraged-authorization
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;
use crate::field::oscal_ssp::DateAuthorized;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct LeveragedAuthorization {
	pub date_authorized: DateAuthorized,
	/// Leveraged Authorization Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope and can be used to reference this leveraged authorization elsewhere in this or other OSCAL instances. The locally defined UUID of the leveraged authorization can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub remarks: Option<Remarks>,
	/// title field
	/// A human readable name for the leveraged authorization in the context of the system.
	pub title: String,
	/// party-uuid field
	/// A machine-oriented identifier reference to the party that manages the leveraged system.
	pub party_uuid: UuidDatatype,
	pub props: Option<Vec<Property>>,
	pub links: Option<Vec<Link>>,
}
