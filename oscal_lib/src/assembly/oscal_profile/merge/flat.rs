/// Flat
/// Use the flat structuring method.
/// $id: #assembly_oscal-profile_merge_flat_flat
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Flat {
}
