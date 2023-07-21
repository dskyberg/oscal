use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::{NCName, Properties};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Role {
    /// A human-oriented, locally unique identifier with cross-instance scope that can be used to reference this defined role elsewhere in this or other OSCAL instances.
    pub id: NCName,
    /// A name given to the role, which may be used by a tool for display and navigation.
    pub title: String,
    /// A short common name, abbreviation, or acronym for the role.
    pub short_name: Option<String>,
    /// A summary of the role's purpose and associated responsibilities.
    pub description: Option<String>,
    /// An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair
    pub props: Option<Properties>,
    /// A reference to a local or remote resource
    pub links: Option<Vec<String>>,
    /// Additional commentary on the containing object.
    pub remarks: Option<String>, // TODO: multi-line markup
}

pub type Roles = Vec<Role>;
