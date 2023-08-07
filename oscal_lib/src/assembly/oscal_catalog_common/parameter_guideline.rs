/// Guideline
/// A prose statement that provides a recommendation for the use of a parameter.
/// $id: #assembly_oscal-catalog-common_parameter-guideline
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ParameterGuideline {
	/// Guideline Text
	/// Prose permits multiple paragraphs, lists, tables etc.
	pub prose: String,
}
