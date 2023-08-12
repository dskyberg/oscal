pub use combination_rule::*;
pub use custom_grouping::*;
pub use flat::*;


pub mod combination_rule;
pub mod custom_grouping;
pub mod flat;

/// Merge controls
/// A Merge element provides structuring directives that drive how controls are organized after resolution.
/// $id: #assembly_oscal-profile_merge
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::BooleanDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Merge {
	/// Combination rule
	/// A Combine element defines how to combine multiple (competing) versions of the same control.
	pub combine: Option<CombinationRule>,
	/// Custom grouping
	/// A Custom element frames a structure for embedding represented controls in resolution.
	pub custom: Option<CustomGrouping>,
	/// Flat
	/// Use the flat structuring method.
	pub flat: Option<Flat>,
	/// As-Is Structuring Directive
	/// An As-is element indicates that the controls should be structured in resolution as they are structured in their source catalogs. It does not contain any elements or attributes.
	pub as_is: Option<BooleanDatatype>,
}
