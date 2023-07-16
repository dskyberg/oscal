use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// A resolvable URL reference to a resource.
    pub href: String,
    /// Describes the type of relationship provided by the link. This can be an indicator of the link's purpose.
    pub rel: Option<String>,
    /// pecifies a media type as defined by the Internet Assigned Numbers Authority (IANA) Media Types Registry
    pub media_type: Option<String>,
    /// A textual label to associate with the link, which may be used for presentation in a tool
    pub text: Option<String>,
}

pub type Links = Vec<Link>;
