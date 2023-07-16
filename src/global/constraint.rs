use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::ConstraintTests;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub description: Option<String>,
    pub tests: Option<ConstraintTests>,
}

pub type Constraints = Vec<Constraint>;
