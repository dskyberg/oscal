/// System User
/// A type of user that interacts with the system based on an associated role.
/// $id: #assembly_oscal-implementation-common_system-user
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::AuthorizedPrivilege;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::StringDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;
use crate::field::oscal_metadata::RoleId;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SystemUser {
	pub props: Option<Vec<Property>>,
	pub role_ids: Option<Vec<RoleId>>,
	/// User Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this user class elsewhere in this or other OSCAL instances. The locally defined UUID of the system user can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub authorized_privileges: Option<Vec<AuthorizedPrivilege>>,
	/// User Description
	/// A summary of the user's purpose within the system.
	pub description: Option<String>,
	/// User Title
	/// A name given to the user, which may be used by a tool for display and navigation.
	pub title: Option<String>,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	/// User Short Name
	/// A short common name, abbreviation, or acronym for the user.
	pub short_name: Option<StringDatatype>,
}
