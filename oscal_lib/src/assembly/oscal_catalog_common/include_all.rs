/// Include All
/// Include all controls from the imported catalog or profile resources.
/// $id: #assembly_oscal-catalog-common_include-all
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct IncludeAll {
}
