use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guideline {
    pub prose: String,
}

pub type Guidelines = Vec<Guideline>;
