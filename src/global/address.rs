use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressType {
    Home,
    Work,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Address {
    ///  Indicates the type of address
    #[serde(rename = "type")]
    address_type: Option<AddressType>,
    addr_lines: Option<Vec<String>>,
    /// City, town or geographical region for the mailing address
    pub city: Option<String>,
    /// State, province or analogous geographical region for mailing address
    pub state: Option<String>,
    /// Postal or ZIP code for mailing address
    pub postal_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code for the mailing address
    pub country: Option<String>, // TODO: validate as 2 char country abbrev.
}

pub type Addresses = Vec<Address>;
