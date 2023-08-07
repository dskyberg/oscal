pub use property_name::*;


pub mod property_name;

/// Property
/// An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values.
/// $id: #assembly_oscal-metadata_property
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;
use crate::definitions::TokenDatatype;
use crate::definitions::UriDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Property {
	/// Property Name
	/// A textual label that uniquely identifies a specific attribute, characteristic, or quality of the property's containing object.
	pub name: PropertyName,
	/// Property Value
	/// Indicates the value of the attribute, characteristic, or quality.
	pub value: StringDatatype,
	/// Property Class
	/// A textual label that provides a sub-type or characterization of the property's name. This can be used to further distinguish or discriminate between the semantics of multiple properties of the same object with the same name and ns.
	pub class: Option<TokenDatatype>,
	pub remarks: Option<Remarks>,
	/// Property Namespace
	/// A namespace qualifying the property's name. This allows different organizations to associate distinct semantics with the same name.
	pub ns: Option<UriDatatype>,
	/// Property Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this defined property elsewhere in this or other OSCAL instances. This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: Option<UuidDatatype>,
}
