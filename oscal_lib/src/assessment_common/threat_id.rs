/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/threat_id.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, URIDatatype, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ThreatId {
    /// "enum": [
    ///    "http://fedramp.gov",
    ///    "http://fedramp.gov/ns/oscal"
    /// ]
    pub system: URIDatatype,
    pub href: Option<URIReferenceDatatype>,
    pub id: URIDatatype,
}

impl SchemaConstraint for ThreatId {
    fn constraint_title() -> &'static str {
        "Threat ID"
    }
    fn constraint_description() -> &'static str {
        r#"A pointer, by ID, to an externally-defined threat."#
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-assessment-common_threat-id"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:threat-id"
    }
}
