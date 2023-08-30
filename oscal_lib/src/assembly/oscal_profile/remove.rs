/// Removal
/// Specifies objects to be removed from a control based on specific aspects of the object that must all match.
/// $id: #assembly_oscal-profile_remove
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::TokenDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Remove {
	/// Item Name Reference
	/// Identify items to remove by the name of the item's information element name, e.g. title or prop
	pub by_item_name: Option<TokenDatatype>,
	/// Reference by class
	/// Identify items to remove by matching their class.
	pub by_class: Option<TokenDatatype>,
	/// Reference by (assigned) name
	/// Identify items to remove by matching their assigned name
	pub by_name: Option<TokenDatatype>,
	/// Reference by ID
	/// Identify items to remove indicated by their id.
	pub by_id: Option<TokenDatatype>,
	/// Item Namespace Reference
	/// Identify items to remove by the item's ns, which is the namespace associated with a part, or prop.
	pub by_ns: Option<TokenDatatype>,
}
