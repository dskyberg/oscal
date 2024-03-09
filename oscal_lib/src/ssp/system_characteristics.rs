/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_characteristics.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::system_id::SystemId,
    metadata::{Link, Property, Remarks, ResponsibleParty},
    SchemaElement, StringDatatype,
};

use super::{
    authorization_boundary::AuthorizationBoundary, data_flow::DataFlow,
    network_architecture::NetworkArchitecture, security_impact_level::SecurityImpactLevel,
    status::Status, system_authorization_date::SystemAuthorizationDate,
    system_information::SystemInformation,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemCharacteristics {
    pub system_ids: Vec<SystemId>,
    pub system_name: StringDatatype,
    pub system_name_short: Option<StringDatatype>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub date_authorized: Option<SystemAuthorizationDate>,
    pub security_sensitivity_level: StringDatatype,
    pub system_information: SystemInformation,
    pub security_impact_level: SecurityImpactLevel,
    pub status: Status,
    pub authorization_boundary: AuthorizationBoundary,
    pub network_architecture: Option<NetworkArchitecture>,
    pub data_flow: Option<DataFlow>,
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for SystemCharacteristics {
    fn schema_title() -> &'static str {
        "System Characteristics"
    }
    fn schema_description() -> &'static str {
        r#"Contains the characteristics of the system, such as its name, purpose, and security impact level."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_system-characteristics")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-characteristics"
    }
}
