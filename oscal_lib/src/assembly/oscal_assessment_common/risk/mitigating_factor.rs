/// Mitigating Factor
/// Describes an existing mitigating factor that may affect the overall determination of the risk, with an optional link to an implementation statement in the SSP.
/// $id: #assembly_oscal-assessment-common_risk_mitigating-factor_mitigating-factor
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::SubjectReference;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct MitigatingFactor {
	/// Implementation UUID
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this implementation statement elsewhere in this or other OSCAL instancess. The locally defined UUID of the implementation statement can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub implementation_uuid: Option<UuidDatatype>,
	/// Mitigating Factor Description
	/// A human-readable description of this mitigating factor.
	pub description: String,
	pub props: Option<Vec<Property>>,
	pub subjects: Option<Vec<SubjectReference>>,
	/// Mitigating Factor Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this mitigating factor elsewhere in this or other OSCAL instances. The locally defined UUID of the mitigating factor can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub links: Option<Vec<Link>>,
}
