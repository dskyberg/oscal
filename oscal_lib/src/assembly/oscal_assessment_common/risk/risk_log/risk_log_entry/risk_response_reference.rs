/// Risk Response Reference
/// Identifies an individual risk response that this log entry is for.
/// $id: #assembly_oscal-assessment-common_risk_risk-log_risk-log_risk-log-entry_risk-log-entry_risk-response-reference_risk-response-reference
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::RelatedTask;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct RiskResponseReference {
	pub related_tasks: Option<Vec<RelatedTask>>,
	pub props: Option<Vec<Property>>,
	/// Response Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a unique risk response.
	pub response_uuid: UuidDatatype,
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
}
