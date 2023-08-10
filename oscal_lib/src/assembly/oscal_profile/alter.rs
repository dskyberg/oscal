/// Alteration
/// An Alter element specifies changes to be made to an included control when a profile is resolved.
/// $id: #assembly_oscal-profile_alter
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_profile::Add;
use crate::assembly::oscal_profile::Remove;
use crate::definitions::TokenDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Alter {
	pub removes: Option<Vec<Remove>>,
	/// Control Identifier Reference
	/// A human-oriented identifier reference to a control with a corresponding id value. When referencing an externally defined control, the Control Identifier Reference must be used in the context of the external / imported OSCAL instance (e.g., uri-reference).
	pub control_id: TokenDatatype,
	pub adds: Option<Vec<Add>>,
}
