/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/property.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype, TokenDatatype, URIDatatype, UUIDDatatype};

use super::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Property {
    /// enum: ["marking"]
    pub name: TokenDatatype,
    pub uuid: Option<UUIDDatatype>,
    pub ns: Option<URIDatatype>,
    pub value: StringDatatype,
    pub class: Option<TokenDatatype>,
    pub remarks: Option<Remarks>,
}

impl Property {
    pub fn new(name: TokenDatatype, value: StringDatatype) -> Self {
        Self {
            name,
            uuid: None,
            ns: None,
            value,
            class: None,
            remarks: None,
        }
    }

    pub fn with_ns(mut self, ns: URIDatatype) -> Self {
        self.ns = Some(ns);
        self
    }
    pub fn with_class(mut self, class: TokenDatatype) -> Self {
        self.class = Some(class);
        self
    }
    pub fn with_remarks(mut self, remarks: super::remarks::Remarks) -> Self {
        self.remarks = Some(remarks);
        self
    }
}

impl SchemaElement for Property {
    fn schema_title() -> &'static str {
        "Property"
    }
    fn schema_description() -> &'static str {
        r#"An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_property")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:property"
    }
}
