/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/set_parameter.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaElement, StringDatatype, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SetParameter {
    pub param_id: TokenDatatype,
    pub values: Vec<StringDatatype>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for SetParameter {
    fn schema_title() -> &'static str {
        "Set Parameter Value"
    }
    fn schema_description() -> &'static str {
        r#"Identifies the parameter that will be set by the enclosed value."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-implementation-common_set-parameter")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:set-parameter"
    }
}
