pub use part_name::*;


pub mod part_name;

/// Assessment Part
/// A partition of an assessment plan or results or a child of another part.
/// $id: #assembly_oscal-assessment-common_assessment-part
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::definitions::UriDatatype;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentPart {
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Part Text
	/// Permits multiple paragraphs, lists, tables etc.
	pub prose: Option<String>,
	/// Part Class
	/// A textual label that provides a sub-type or characterization of the part's name. This can be used to further distinguish or discriminate between the semantics of multiple parts of the same control with the same name and ns.
	pub class: Option<TokenDatatype>,
	/// Part Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this part elsewhere in this or other OSCAL instances. The locally defined UUID of the part can be used to reference the data item locally or globally (e.g., in an ported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: Option<UuidDatatype>,
	pub parts: Option<Vec<AssessmentPart>>,
	/// Part Name
	/// A textual label that uniquely identifies the part's semantic type.
	pub name: PartName,
	/// Part Namespace
	/// A namespace qualifying the part's name. This allows different organizations to associate distinct semantics with the same name.
	pub ns: Option<UriDatatype>,
	/// Part Title
	/// A name given to the part, which may be used by a tool for display and navigation.
	pub title: Option<String>,
}
