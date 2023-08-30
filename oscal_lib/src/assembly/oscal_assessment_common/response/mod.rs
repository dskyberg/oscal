pub use required_asset::*;


pub mod required_asset;

/// Risk Response
/// Describes either recommended or an actual plan for addressing the risk.
/// $id: #assembly_oscal-assessment-common_response
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::Origin;
use crate::assembly::oscal_assessment_common::Task;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Response {
	/// Remediation Intent
	/// Identifies whether this is a recommendation, such as from an assessor or tool, or an actual plan accepted by the system owner.
	pub lifecycle: TokenDatatype,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Response Description
	/// A human-readable description of this response plan.
	pub description: String,
	pub tasks: Option<Vec<Task>>,
	/// Remediation Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this remediation elsewhere in this or other OSCAL instances. The locally defined UUID of the risk response can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub required_assets: Option<Vec<RequiredAsset>>,
	pub remarks: Option<Remarks>,
	pub origins: Option<Vec<Origin>>,
	/// Response Title
	/// The title for this response activity.
	pub title: String,
}
