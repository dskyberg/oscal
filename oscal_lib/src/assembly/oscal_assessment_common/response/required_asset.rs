/// Required Asset
/// Identifies an asset required to achieve remediation.
/// $id: #assembly_oscal-assessment-common_response_required-asset_required-asset
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::SubjectReference;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct RequiredAsset {
	/// Required Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this required asset elsewhere in this or other OSCAL instances. The locally defined UUID of the asset can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Description of Required Asset
	/// A human-readable description of this required asset.
	pub description: String,
	pub props: Option<Vec<Property>>,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	/// Title for Required Asset
	/// The title for this required asset.
	pub title: Option<String>,
	pub subjects: Option<Vec<SubjectReference>>,
}
