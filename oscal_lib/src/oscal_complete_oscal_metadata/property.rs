/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/property.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TokenDatatype, URIDatatype, UUIDDatatype};

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
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<super::remarks::Remarks>,
}

impl SchemaConstraint for Property {
    fn constraint_title() -> &'static str {
        "Property"
    }
    fn constraint_description() -> &'static str {
        r#"An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_property"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:property"
    }
}
