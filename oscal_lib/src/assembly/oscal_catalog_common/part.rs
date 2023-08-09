/// Part
/// A partition of a control's definition or a child of another part.
/// $id: #assembly_oscal-catalog-common_part
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::definitions::UriDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Part {
	/// Part Class
	/// A textual label that provides a sub-type or characterization of the part's name. This can be used to further distinguish or discriminate between the semantics of multiple parts of the same control with the same name and ns.
	pub class: Option<TokenDatatype>,
	/// Part Title
	/// A name given to the part, which may be used by a tool for display and navigation.
	pub title: Option<String>,
	/// Part Namespace
	/// A namespace qualifying the part's name. This allows different organizations to associate distinct semantics with the same name.
	pub ns: Option<UriDatatype>,
	/// Part Name
	/// A textual label that uniquely identifies the part's semantic type.
	pub name: TokenDatatype,
	/// Part Identifier
	/// A human-oriented, locally unique identifier with cross-instance scope that can be used to reference this defined part elsewhere in this or other OSCAL instances. When referenced from another OSCAL instance, this identifier must be referenced in the context of the containing resource (e.g., import-profile). This id should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub id: Option<TokenDatatype>,
	pub parts: Option<Vec<Part>>,
	pub links: Option<Vec<Link>>,
	/// Part Text
	/// Permits multiple paragraphs, lists, tables etc.
	pub prose: Option<String>,
	pub props: Option<Vec<Property>>,
}
