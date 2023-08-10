/// Profile
/// Each OSCAL profile is defined by a Profile element
/// $id: #assembly_oscal-profile_profile
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::BackMatter;
use crate::assembly::oscal_metadata::Metadata;
use crate::assembly::oscal_profile::Import;
use crate::assembly::oscal_profile::Merge;
use crate::assembly::oscal_profile::Modify;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Profile {
	/// Profile Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this profile elsewhere in this or other OSCAL instances. The locally defined UUID of the profile can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance).This identifier should be assigned per-subject, which means it should be consistently used to identify the same profile across revisions of the document.
	pub uuid: UuidDatatype,
	pub back_matter: Option<BackMatter>,
	pub imports: Vec<Import>,
	pub merge: Option<Merge>,
	pub metadata: Metadata,
	pub modify: Option<Modify>,
}
