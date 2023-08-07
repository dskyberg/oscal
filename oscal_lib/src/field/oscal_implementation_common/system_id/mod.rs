pub use identification_system_type::*;


pub mod identification_system_type;

/// System Identification
/// A human-oriented, globally unique identifier with cross-instance scope that can be used to reference this system identification property elsewhere in this or other OSCAL instances. When referencing an externally defined system identification, the system identification must be used in the context of the external / imported OSCAL instance (e.g., uri-reference). This string should be assigned per-subject, which means it should be consistently used to identify the same system across revisions of the document.
/// $id: #field_oscal-implementation-common_system-id
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SystemId {
	/// Identification System Type
	/// Identifies the identification system from which the provided identifier was assigned.
	pub identifier_type: Option<IdentificationSystemType>,
	pub id: StringDatatype,
}
