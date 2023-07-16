use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::part::Parts;
use crate::global::{Links, Params, Props};

// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub id: String,
    pub class: Option<String>,
    pub title: String,
    pub params: Option<Params>,
    pub props: Option<Props>,
    pub links: Option<Links>,
    pub parts: Option<Parts>,
    pub controls: Option<Controls>,
}

pub type Controls = Vec<Control>;
