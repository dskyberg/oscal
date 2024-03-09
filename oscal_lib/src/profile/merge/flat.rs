use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Flat {}

impl SchemaElement for Flat {
    fn schema_title() -> &'static str {
        "Flat"
    }
    fn schema_description() -> &'static str {
        "Use the flat structuring method."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:merge:flat"
    }
}
