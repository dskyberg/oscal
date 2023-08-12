/// Relation
/// Describes the type of relationship provided by the link. This can be an indicator of the link's purpose.
/// $id: #assembly_oscal-metadata_link_relation_relation
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum Relation {
	// orig: reference
	Reference,
	// orig: resolution-source
	ResolutionSource,
	// orig: homepage
	Homepage,
	// orig: logo
	Logo,
	// orig: related
	Related,
	// orig: corresp
	Corresp,
}
