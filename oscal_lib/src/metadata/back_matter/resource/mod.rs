use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{DocumentId, Property, Remarks},
    SchemaElement, UUIDDatatype,
};

use self::{base64::Base64, citation::Citation, resource_link::ResourceLink};

pub mod base64;
pub mod citation;
pub mod resource_link;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Resource {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub document_ids: Option<Vec<DocumentId>>,
    pub citation: Option<Citation>,
    pub rlinks: Option<Vec<ResourceLink>>,
    pub base64: Option<Base64>,
    pub remarks: Option<Option<Remarks>>,
}

impl SchemaElement for Resource {
    fn schema_title() -> &'static str {
        "Resource"
    }
    fn schema_description() -> &'static str {
        "A resource associated with content in the containing document. A resource may be directly included in the document base64 encoded or may point to one or more equivalent internet resources."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter"
    }
}
