/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_information.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    SchemaConstraint,
};

use self::information_type::InformationType;

pub mod information_type;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemInformation {
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub information_types: Vec<InformationType>,
}

impl SchemaConstraint for SystemInformation {
    fn constraint_title() -> &'static str {
        "System Information"
    }
    fn constraint_description() -> &'static str {
        r#"Contains details about all information types that are stored, processed, or transmitted by the system, such as privacy information, and those defined in NIST SP 800-60."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_system-information"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-information"
    }
}
