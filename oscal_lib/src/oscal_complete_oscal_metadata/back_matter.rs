/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/back_matter.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Base64Datatype, SchemaConstraint, StringDatatype, URIReferenceDatatype, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Citation {
    pub text: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<super::link::Link>>,
}
impl SchemaConstraint for Citation {
    fn constraint_title() -> &'static str {
        "Citation"
    }

    fn constraint_description() -> &'static str {
        "A citation consisting of end note text and optional structured bibliographic data."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter:resource:citation"
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/citation"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceLink {
    href: URIReferenceDatatype,
    media_type: Option<StringDatatype>,
    /// "#field_oscal-metadata_hash"
    hashes: Option<Vec<super::hash::Hash>>,
}
impl SchemaConstraint for ResourceLink {
    fn constraint_title() -> &'static str {
        "Resource link"
    }

    fn constraint_description() -> &'static str {
        "A resolvable URI reference to a resource."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter:resource:rlink"
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/rlink"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Base64 {
    filename: Option<URIReferenceDatatype>,
    media_type: Option<StringDatatype>,
    value: Base64Datatype,
}

impl SchemaConstraint for Base64 {
    fn constraint_title() -> &'static str {
        "Base64"
    }

    fn constraint_description() -> &'static str {
        "The Base64 alphabet in RFC 2045 - aligned with XSD."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter:resource:bse64"
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/base64"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatterResource {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<super::property::Property>>,
    /// "#field_oscal-metadata_document-id"
    pub document_ids: Option<Vec<super::document_id::DocumentId>>,
    pub citation: Option<Citation>,
    pub rlinks: Option<Vec<ResourceLink>>,
    pub base64: Option<Base64>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<Option<super::remarks::Remarks>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackMatter {
    pub resources: Vec<BackMatterResource>,
}

impl SchemaConstraint for BackMatter {
    fn constraint_title() -> &'static str {
        "Back matter"
    }
    fn constraint_description() -> &'static str {
        r#"A collection of resources, which may be included directly or by reference."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = include_str!("test/back_matter.json");
        let result = serde_json::from_str::<BackMatter>(json).expect("oops");
        dbg!(result);
    }
}
