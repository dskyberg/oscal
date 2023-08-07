pub use local_definitions::*;


pub mod local_definitions;

/// Security Assessment Results (SAR)
/// Security assessment results, such as those provided by a FedRAMP assessor in the FedRAMP Security Assessment Report.
/// $id: #assembly_oscal-ar_assessment-results
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_ar::ImportAp;
use crate::assembly::oscal_ar::Result;
use crate::assembly::oscal_metadata::BackMatter;
use crate::assembly::oscal_metadata::Metadata;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentResults {
	pub import_ap: ImportAp,
	pub results: Vec<Result>,
	/// Assessment Results Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this assessment results instance in this or other OSCAL instances. The locally defined UUID of the assessment result can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub back_matter: Option<BackMatter>,
	pub metadata: Metadata,
	/// Local Definitions
	/// Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP.
	pub local_definitions: Option<LocalDefinitions>,
}
