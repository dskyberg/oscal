use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    MetadataRemarks, SchemaConstraint, StringDatatype, TokenDatatype, URIDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataProperty {
    pub name: TokenDatatype,
    pub uuid: Option<UUIDDatatype>,
    pub ns: Option<URIDatatype>,
    pub value: StringDatatype,
    pub class: Option<TokenDatatype>,
    pub remarks: Option<MetadataRemarks>,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "props[@name]"}
    ].
    "allowed-values": [
        "marking"
    ]
}
 */

impl SchemaConstraint for MetadataProperty {
    fn constraint_title() -> &'static str {
        "Property"
    }
    fn constraint_description() -> &'static str {
        "An attribute, characteristic, or quality of the containing object expressed as a namespace qualified name/value pair. The value of a property is a simple scalar value, which may be expressed as a list of values."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_property"
    }
    fn schema_path() -> &'static str {
        "prop"
    }
}
