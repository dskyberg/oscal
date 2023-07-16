use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintTest {
    pub expression: String,
    pub remarks: Option<String>,
}

pub type ConstraintTests = Vec<ConstraintTest>;
