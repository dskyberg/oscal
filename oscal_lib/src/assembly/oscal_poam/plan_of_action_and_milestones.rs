/// Plan of Action and Milestones (POA&M)
/// A plan of action and milestones which identifies initial and residual risks, deviations, and disposition, such as those required by FedRAMP.
/// $id: #assembly_oscal-poam_plan-of-action-and-milestones
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::ImportSsp;
use crate::assembly::oscal_assessment_common::Observation;
use crate::assembly::oscal_assessment_common::Risk;
use crate::assembly::oscal_metadata::BackMatter;
use crate::assembly::oscal_metadata::Metadata;
use crate::assembly::oscal_poam::LocalDefinitions;
use crate::assembly::oscal_poam::PoamItem;
use crate::definitions::UuidDatatype;
use crate::field::oscal_implementation_common::SystemId;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct PlanOfActionAndMilestones {
	pub risks: Option<Vec<Risk>>,
	pub system_id: Option<SystemId>,
	pub local_definitions: Option<LocalDefinitions>,
	pub import_ssp: Option<ImportSsp>,
	pub observations: Option<Vec<Observation>>,
	pub back_matter: Option<BackMatter>,
	/// POA&M Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with instancescope that can be used to reference this POA&M instance in this OSCAL instance. This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub metadata: Metadata,
	pub poam_items: Vec<PoamItem>,
}
