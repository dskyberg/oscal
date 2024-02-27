/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/port_range.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{NonNegativeIntegerDatatype, SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PortRange {
    pub start: NonNegativeIntegerDatatype,
    pub end: NonNegativeIntegerDatatype,
    /// enum: [
    ///     "TCP",
    ///     "UDP"
    /// ]
    pub transport: TokenDatatype,
}

impl SchemaConstraint for PortRange {
    fn constraint_title() -> &'static str {
        "Port Range"
    }
    fn constraint_description() -> &'static str {
        r#"Where applicable this is the IPv4 port range on which the service operates."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_port-range"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:port-range"
    }
}
