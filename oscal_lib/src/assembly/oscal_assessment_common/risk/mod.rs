pub use mitigating_factor::*;
pub use related_observation::*;
pub use risk_log::*;

pub mod mitigating_factor;
pub mod related_observation;
pub mod risk_log;
pub mod risk_log_entry;
pub mod risk_response_reference;

/// Identified Risk
/// An identified risk.
/// $id: #assembly_oscal-assessment-common_risk
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::Characterization;
use crate::assembly::oscal_assessment_common::Origin;
use crate::assembly::oscal_assessment_common::Response;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::DateTimeWithTimezoneDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_assessment_common::RiskStatus;
use crate::field::oscal_assessment_common::ThreatId;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Risk {
    /// Risk Description
    /// A human-readable summary of the identified risk, to include a statement of how the risk impacts the system.
    pub description: String,
    /// Risk Resolution Deadline
    /// The date/time by which the risk must be resolved.
    pub deadline: Option<DateTimeWithTimezoneDatatype>,
    /// Risk Title
    /// The title for this risk.
    pub title: String,
    pub characterizations: Option<Vec<Characterization>>,
    pub threat_ids: Option<Vec<ThreatId>>,
    pub related_observations: Option<Vec<RelatedObservation>>,
    pub status: RiskStatus,
    pub mitigating_factors: Option<Vec<MitigatingFactor>>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remediations: Option<Vec<Response>>,
    /// Risk Universally Unique Identifier
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this risk elsewhere in this or other OSCAL instances. The locally defined UUID of the risk can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: UuidDatatype,
    /// Risk Log
    /// A log of all risk-related tasks taken.
    pub risk_log: Option<RiskLog>,
    pub origins: Option<Vec<Origin>>,
    /// Risk Statement
    /// An summary of impact for how the risk affects the system.
    pub statement: String,
}
