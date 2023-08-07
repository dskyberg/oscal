/// Publication metadata
/// Provides information about the publication and availability of the containing document.
/// $id: #assembly_oscal-metadata_metadata
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Location;
use crate::assembly::oscal_metadata::Party;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleParty;
use crate::assembly::oscal_metadata::Revision;
use crate::assembly::oscal_metadata::Role;
use crate::field::oscal_metadata::DocumentId;
use crate::field::oscal_metadata::LastModified;
use crate::field::oscal_metadata::OscalVersion;
use crate::field::oscal_metadata::Published;
use crate::field::oscal_metadata::Remarks;
use crate::field::oscal_metadata::Version;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Metadata {
	pub parties: Option<Vec<Party>>,
	pub props: Option<Vec<Property>>,
	pub document_ids: Option<Vec<DocumentId>>,
	pub last_modified: LastModified,
	pub links: Option<Vec<Link>>,
	pub published: Option<Published>,
	pub responsible_parties: Option<Vec<ResponsibleParty>>,
	pub version: Version,
	pub oscal_version: OscalVersion,
	pub revisions: Option<Vec<Revision>>,
	/// Document Title
	/// A name given to the document, which may be used by a tool for display and navigation.
	pub title: String,
	pub remarks: Option<Remarks>,
	pub roles: Option<Vec<Role>>,
	pub locations: Option<Vec<Location>>,
}
