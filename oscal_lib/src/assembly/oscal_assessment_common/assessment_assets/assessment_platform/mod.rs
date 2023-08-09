pub use uses_component::*;


pub mod uses_component;

/// Assessment Platform
/// Used to represent the toolset used to perform aspects of the assessment.
/// $id: #assembly_oscal-assessment-common_assessment-assets_assessment-platform_assessment-platform
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentPlatform {
	pub uses_components: Option<Vec<UsesComponent>>,
	pub props: Option<Vec<Property>>,
	/// Assessment Platform Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this assessment platform elsewhere in this or other OSCAL instances. The locally defined UUID of the assessment platform can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
	/// Assessment Platform Title
	/// The title or name for the assessment platform.
	pub title: Option<String>,
}
