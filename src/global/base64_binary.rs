use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64Binary {
    pub filename: Option<String>,
    pub media_type: Option<String>,
    pub value: Option<String>,
}
