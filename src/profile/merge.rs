use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::{ControlGroups, Controls};

/// How clashing controls should be handled
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombinationMethod {
    #[serde(rename = "use-first")]
    UseFirst,
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "keep")]
    Keep,
}

/// A Combine element defines how to combine multiple (competing) versions of the same control.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinationRule {
    pub method: Option<CombinationMethod>,
}

/// Use the flat structuring method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flat {}

/// A Custom element frames a structure for embedding represented controls in resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomGrouping {
    pub groups: Option<ControlGroups>,
    #[serde(rename = "insert-controls")]
    pub insert_controls: Option<Controls>,
}

/// A Merge element provides structuring directives that drive how controls are organized after resolution.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merge {
    pub combine: Option<CombinationRule>,
    pub flat: Option<Flat>,
    #[serde(rename = "as-is")]
    pub as_is: Option<bool>,
}
