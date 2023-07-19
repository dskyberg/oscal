use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::Resources;
// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackMatter {
    resources: Option<Resources>,
}
