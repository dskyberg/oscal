use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TelephoneNumberType {
    Home,
    Office,
    Mobile,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelephoneNumber {
    /// Indicates the type of phone number.
    #[serde(rename = "type")]
    pub _type: Option<TelephoneNumberType>,
    pub number: Option<String>,
}

pub type TelephoneNumbers = Vec<TelephoneNumber>;
