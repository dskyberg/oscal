/// Parameter Setting
/// A parameter setting, to be propagated to points of insertion
/// $id: #assembly_oscal-profile_modify_parameter-setting_parameter-setting
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog_common::ParameterConstraint;
use crate::assembly::oscal_catalog_common::ParameterGuideline;
use crate::assembly::oscal_catalog_common::ParameterSelection;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::field::oscal_catalog_common::ParameterValue;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ParameterSetting {
	pub props: Option<Vec<Property>>,
	pub select: Option<ParameterSelection>,
	/// Parameter Usage Description
	/// Describes the purpose and use of a parameter
	pub usage: Option<String>,
	/// Depends on
	/// **(deprecated)** Another parameter invoking this one. This construct has been deprecated and should not be used.
	pub depends_on: Option<TokenDatatype>,
	pub values: Option<Vec<ParameterValue>>,
	pub links: Option<Vec<Link>>,
	pub constraints: Option<Vec<ParameterConstraint>>,
	pub guidelines: Option<Vec<ParameterGuideline>>,
	/// Parameter Class
	/// A textual label that provides a characterization of the parameter.
	pub class: Option<TokenDatatype>,
	/// Parameter Label
	/// A short, placeholder name for the parameter, which can be used as a substitute for a value if no value is assigned.
	pub label: Option<String>,
	/// Parameter ID
	/// A human-oriented, locally unique identifier with cross-instance scope that can be used to reference this defined parameter elsewhere in this or other OSCAL instances. When referenced from another OSCAL instance, this identifier must be referenced in the context of the containing resource (e.g., import-profile). This id should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub param_id: TokenDatatype,
}
