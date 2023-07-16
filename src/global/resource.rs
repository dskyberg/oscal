use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::global::{Base64Binary, Citation, DocumentIds, Links, Props};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressType {
    Home,
    Work,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uuid: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub props: Option<Props>,
    #[serde(rename = "document-ids")]
    pub document_ids: Option<DocumentIds>,
    pub citation: Option<Citation>,
    pub rlinks: Option<Links>, // TODO: define rlinks - it' different from links
    pub base64: Option<Base64Binary>,
    pub remarks: Option<String>,
}

pub type Resources = Vec<Resource>;
