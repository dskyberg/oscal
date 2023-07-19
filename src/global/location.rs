use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::global::{Address, Links, Properties, TelephoneNumbers};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this defined location elsewhere in this or other OSCAL instances
    pub uuid: Uuid,
    /// A name given to the location, which may be used by a tool for display and navigation.
    pub title: Option<String>,
    /// A postal address for the location.
    pub address: Address, //TODO: define Address
    /// An email address as defined by RFC 5322 Section 3.4.1
    #[serde(rename = "email-addresses")]
    pub email_addresses: Option<Vec<String>>, //TODO: validate as email
    /// Contact number by telephone.
    #[serde(rename = "telephone-numbers")]
    telephone_numbers: Option<TelephoneNumbers>, //TODO: define
    ///The uniform resource locator (URL) for a web site or Internet presence associated with the location.
    pub urls: Option<Vec<String>>,
    /// An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair.
    pub props: Option<Properties>,
    /// A reference to a local or remote resource
    pub links: Option<Links>,
    /// Additional commentary on the containing object.
    pub remarks: Option<Vec<String>>,
}

pub type Locations = Vec<Location>;
