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
pub struct Party {
    uuid: Uuid,
    #[serde(rename = "type")]
    party_type: PartyType,
    name: Option<String>,
    #[serde(rename = "short-name")]
    short_name: Option<String>,
    #[serde(rename = "external-ids")]
    external_ids: Option<Vec<String>>, // TODO: validate as Uri
    props: Option<Properties>,
    links: Option<Vec<String>>,
    #[serde(rename = "email-addresses")]
    email_addresses: Option<Vec<String>>,
    #[serde(rename = "telephone_numbers")]
    telephone_numbers: Option<Vec<String>>,
    addresses: Option<Addresses>,
    #[serde(rename = "location-uuids")]
    location_uuids: Option<Vec<Uuid>>,
    #[serde(rename = "member-of-organizations")]
    member_of_organizations: Option<Vec<Uuid>>,
    remarks: Option<String>,
}

pub type Parties = Vec<Party>;
