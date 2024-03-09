/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_implementation.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::{InventoryItem, SystemComponent, SystemUser},
    metadata::{Link, Property, Remarks},
    SchemaElement,
};

use self::leveraged_authorization::LeveragedAuthorization;

pub mod leveraged_authorization;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemImplementation {
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub leveraged_authorizations: Option<Vec<LeveragedAuthorization>>,
    pub users: Vec<SystemUser>,
    pub components: Vec<SystemComponent>,
    pub inventory_items: Option<Vec<InventoryItem>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for SystemImplementation {
    fn schema_title() -> &'static str {
        "System Implementation"
    }
    fn schema_description() -> &'static str {
        r#"Provides information as to how the system is implemented."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_system-implementation")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-implementation"
    }
}
