/// Order
/// A designation of how a selection of controls in a profile is to be ordered.
/// $id: #assembly_oscal-profile_insert-controls_order_order
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum Order {
	// orig: keep
	Keep,
	// orig: ascending
	Ascending,
	// orig: descending
	Descending,
}
