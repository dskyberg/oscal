use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::Token;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Link {
    /// A resolvable URL reference to a resource.
    pub href: String,
    /// Describes the type of relationship provided by the link. This can be an indicator of the link's purpose.
    pub rel: Option<Token>,
    /// pecifies a media type as defined by the Internet Assigned Numbers Authority (IANA) Media Types Registry
    pub media_type: Option<String>,
    /// A textual label to associate with the link, which may be used for presentation in a tool
    pub text: Option<String>,
}

pub type Links = Vec<Link>;
