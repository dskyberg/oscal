use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::global::{Addresses, Properties};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartyType {
    Person,
    Organization,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Party {
    pub uuid: Uuid,
    #[serde(rename = "type")]
    pub party_type: PartyType,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub external_ids: Option<Vec<String>>, // TODO: validate as Uri
    pub props: Option<Properties>,
    pub links: Option<Vec<String>>,
    pub email_addresses: Option<Vec<String>>,
    pub telephone_numbers: Option<Vec<String>>,
    pub addresses: Option<Addresses>,
    pub location_uuids: Option<Vec<Uuid>>,
    pub member_of_organizations: Option<Vec<Uuid>>,
    pub remarks: Option<String>,
}

pub type Parties = Vec<Party>;
