use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Links, Props};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub text: String,
    pub props: Option<Props>,
    pub links: Option<Links>,
}

pub type Citations = Vec<Citation>;
