pub use assessment_log::*;
pub use attestation_statements::*;
pub use local_definitions::*;

pub mod assessment_log;
pub mod attestation_statements;
pub mod local_definitions;

/// Assessment Result
/// Used by the assessment results and POA&M. In the assessment results, this identifies all of the assessment observations and findings, initial and residual risks, deviations, and disposition. In the POA&M, this identifies initial and residual risks, deviations, and disposition.
/// $id: #assembly_oscal-ar_result
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_ar::Finding;
use crate::assembly::oscal_assessment_common::Observation;
use crate::assembly::oscal_assessment_common::ReviewedControls;
use crate::assembly::oscal_assessment_common::Risk;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::DateTimeWithTimezoneDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Result {
    pub observations: Option<Vec<Observation>>,
    /// end field
    /// Date/time stamp identifying the end of the evidence collection reflected in these results. In a continuous motoring scenario, this may contain the same value as start if appropriate.
    pub end: Option<DateTimeWithTimezoneDatatype>,
    pub risks: Option<Vec<Risk>>,
    pub findings: Option<Vec<Finding>>,
    /// Assessment Log
    /// A log of all assessment-related actions taken.
    pub assessment_log: Option<AssessmentLog>,
    pub links: Option<Vec<Link>>,
    pub attestations: Option<Vec<AttestationStatements>>,
    /// Results Description
    /// A human-readable description of this set of test results.
    pub description: String,
    /// Local Definitions
    /// Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
    pub local_definitions: Option<LocalDefinitions>,
    /// Results Title
    /// The title for this set of results.
    pub title: String,
    pub reviewed_controls: ReviewedControls,
    /// Results Universally Unique Identifier
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this set of results in this or other OSCAL instances. The locally defined UUID of the assessment result can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
    pub uuid: UuidDatatype,
    pub remarks: Option<Remarks>,
    pub props: Option<Vec<Property>>,
    /// start field
    /// Date/time stamp identifying the start of the evidence collection reflected in these results.
    pub start: DateTimeWithTimezoneDatatype,
}
