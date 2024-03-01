/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_information.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, URIDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InformationTypeCategorization {
    /// "enum": [
    ///    "http://doi.org/10.6028/NIST.SP.800-60v2r1"
    /// ]
    pub system: URIDatatype,
    pub information_type_ids: Option<Vec<StringDatatype>>,
}

impl SchemaConstraint for InformationTypeCategorization {
    fn constraint_title() -> &'static str {
        "Information Type Categorization"
    }
    fn constraint_description() -> &'static str {
        "A set of information type identifiers qualified by the given identification system used, such as NIST SP 800-60."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_system-information_information-type_information-type-characterization"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-information:information-type:information-type-characterization"
    }
}
