use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Links, Properties};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub text: String,
    pub props: Option<Properties>,
    pub links: Option<Links>,
}

pub type Citations = Vec<Citation>;