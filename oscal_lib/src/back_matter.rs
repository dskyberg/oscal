use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Base64Datatype, MetadataDocumentId, MetadataHash, MetadataLink, MetadataProperty,
    MetadataRemarks, SchemaConstraint, StringDatatype, URIReferenceDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceBase64 {
    pub filename: Option<URIReferenceDatatype>,
    pub media_type: Option<StringDatatype>,
    pub value: Base64Datatype,
}

impl SchemaConstraint for ResourceBase64 {
    fn constraint_title() -> &'static str {
        "Base64"
    }
    fn constraint_description() -> &'static str {
        "The Base64 alphabet in RFC 2045 - aligned with XSD."
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "back-matter/resource/base64"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceLink {
    pub href: URIReferenceDatatype,
    pub media_type: Option<StringDatatype>,
    pub hashes: Option<Vec<MetadataHash>>,
}

impl SchemaConstraint for ResourceLink {
    fn constraint_title() -> &'static str {
        "Resource link"
    }
    fn constraint_description() -> &'static str {
        "A pointer to an external resource with an optional hash for verification and change detection."
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "back-matter/resource/rlink"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Citation {
    pub text: String,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
}

impl SchemaConstraint for Citation {
    fn constraint_title() -> &'static str {
        "Citation"
    }
    fn constraint_description() -> &'static str {
        "A citation consisting of end note text and optional structured bibliographic data."
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "back-matter/resource/citation"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatterResource {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: Option<String>,
    pub props: Option<Vec<MetadataProperty>>,
    pub document_ids: Option<Vec<MetadataDocumentId>>,
    pub citation: Option<Citation>,
    pub rlinks: Option<Vec<ResourceLink>>,
    pub bse64: Option<ResourceBase64>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for BackMatterResource {
    fn constraint_title() -> &'static str {
        "Resource"
    }
    fn constraint_description() -> &'static str {
        "A resource associated with content in the containing document. A resource may be directly included in the document base64 encoded or may point to one or more equivalent internet resources."
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "back-matter/resource"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatter {}

impl SchemaConstraint for BackMatter {
    fn constraint_title() -> &'static str {
        "Back matter"
    }
    fn constraint_description() -> &'static str {
        "A collection of resources, which may be included directly or by reference."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter"
    }
    fn schema_path() -> &'static str {
        "back-matter"
    }
}
