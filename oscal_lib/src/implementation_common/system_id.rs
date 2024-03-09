/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/system_id.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype, URIDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemId {
    /// "enum": [
    ///    "https://fedramp.gov",
    ///    "http://fedramp.gov/ns/oscal",
    ///    "https://ietf.org/rfc/rfc4122",
    ///    "http://ietf.org/rfc/rfc4122"
    /// ]
    pub identifier_type: Option<URIDatatype>,
    pub id: StringDatatype,
}

impl SchemaElement for SystemId {
    fn schema_title() -> &'static str {
        "System Identification"
    }
    fn schema_description() -> &'static str {
        r#"A human-oriented, globally unique identifier with cross-instance scope that can be used to reference this system identification property elsewhere in this or other OSCAL instances. When referencing an externally defined system identification, the system identification must be used in the context of the external / imported OSCAL instance (e.g., uri-reference). This string should be assigned per-subject, which means it should be consistently used to identify the same system across revisions of the document."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-implementation-common_system-id")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-id"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = r##"{
            "identifier-type": "https://fedramp.gov",
            "id": "F00000000"
          }"##;

        let result = serde_json::from_str::<SystemId>(json).expect("oops");
        dbg!(result);
    }
}
