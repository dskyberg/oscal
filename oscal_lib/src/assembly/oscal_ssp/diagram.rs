/// Diagram
/// A graphic that provides a visual representation the system, or some aspect of it.
/// $id: #assembly_oscal-ssp_diagram
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Diagram {
	/// Caption
	/// A brief caption to annotate the diagram.
	pub caption: Option<String>,
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
	/// Diagram ID
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this diagram elsewhere in this or other OSCAL instances. The locally defined UUID of the diagram can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub props: Option<Vec<Property>>,
	/// Diagram Description
	/// A summary of the diagram.
	pub description: Option<String>,
}
