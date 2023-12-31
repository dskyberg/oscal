/// Revision History Entry
/// An entry in a sequential list of revisions to the containing document in reverse chronological order (i.e., most recent previous revision first).
/// $id: #assembly_oscal-metadata_revision
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_metadata::LastModified;
use crate::field::oscal_metadata::OscalVersion;
use crate::field::oscal_metadata::Published;
use crate::field::oscal_metadata::Remarks;
use crate::field::oscal_metadata::Version;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Revision {
	pub last_modified: Option<LastModified>,
	pub props: Option<Vec<Property>>,
	pub published: Option<Published>,
	/// Document Title
	/// A name given to the document revision, which may be used by a tool for display and navigation.
	pub title: Option<String>,
	pub links: Option<Vec<Link>>,
	pub oscal_version: Option<OscalVersion>,
	pub remarks: Option<Remarks>,
	pub version: Version,
}
