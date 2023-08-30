/// Parameter
/// Parameters provide a mechanism for the dynamic assignment of value(s) in a control.
/// $id: #assembly_oscal-catalog-common_parameter
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog_common::ParameterConstraint;
use crate::assembly::oscal_catalog_common::ParameterGuideline;
use crate::assembly::oscal_catalog_common::ParameterSelection;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::field::oscal_catalog_common::ParameterValue;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Parameter {
	/// Parameter Identifier
	/// A human-oriented, locally unique identifier with cross-instance scope that can be used to reference this defined parameter elsewhere in this or other OSCAL instances. When referenced from another OSCAL instance, this identifier must be referenced in the context of the containing resource (e.g., import-profile). This id should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub id: TokenDatatype,
	/// Depends on
	/// **(deprecated)** Another parameter invoking this one. This construct has been deprecated and should not be used.
	pub depends_on: Option<TokenDatatype>,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	pub select: Option<ParameterSelection>,
	pub props: Option<Vec<Property>>,
	pub values: Option<Vec<ParameterValue>>,
	/// Parameter Class
	/// A textual label that provides a characterization of the parameter.
	pub class: Option<TokenDatatype>,
	/// Parameter Usage Description
	/// Describes the purpose and use of a parameter
	pub usage: Option<String>,
	pub guidelines: Option<Vec<ParameterGuideline>>,
	/// Parameter Label
	/// A short, placeholder name for the parameter, which can be used as a substitute for a value if no value is assigned.
	pub label: Option<String>,
	pub constraints: Option<Vec<ParameterConstraint>>,
}
