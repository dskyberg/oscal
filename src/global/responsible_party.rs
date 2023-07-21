use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::global::{Links, Properties, Token};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponsibleParty {
    /// A human-oriented identifier reference to roles served by the user.
    pub role_id: Token,

    /// A machine-oriented identifier reference to another party defined in metadata. The UUID of the party in the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance).
    pub party_uuids: Vec<Uuid>,

    /// An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values.
    pub props: Option<Properties>,

    /// An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values.
    pub links: Option<Links>,

    /// Additional commentary on the containing object.
    pub remarks: Option<String>,
}

pub type ResponsibleParties = Vec<ResponsibleParty>;
