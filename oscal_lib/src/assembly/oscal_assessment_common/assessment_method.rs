/// Assessment Method
/// A local definition of a control objective. Uses catalog syntax for control objective and assessment activities.
/// $id: #assembly_oscal-assessment-common_assessment-method
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentPart;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentMethod {
	pub part: AssessmentPart,
	/// Assessment Method Description
	/// A human-readable description of this assessment method.
	pub description: Option<String>,
	pub props: Option<Vec<Property>>,
	/// Assessment Method Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this assessment method elsewhere in this or other OSCAL instances. The locally defined UUID of the assessment method can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
}
