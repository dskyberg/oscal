/// Origin
/// Identifies the source of the finding, such as a tool or person.
/// $id: #assembly_oscal-poam_poam-item_origin_origin
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::OriginActor;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Origin {
	pub actors: Vec<OriginActor>,
}
