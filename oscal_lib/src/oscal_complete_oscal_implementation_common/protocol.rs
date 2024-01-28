/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/protocol.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Protocol {
    pub uuid: Option<UUIDDatatype>,
    pub name: StringDatatype,
    pub title: Option<String>,
    /// "#assembly_oscal-implementation-common_port-range"
    pub port_ranges: Option<Vec<super::port_range::PortRange>>,
}

impl SchemaConstraint for Protocol {
    fn constraint_title() -> &'static str {
        "Service Protocol Information"
    }
    fn constraint_description() -> &'static str {
        r#"Information about the protocol used to provide a service."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_protocol"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:protocol"
    }
}
