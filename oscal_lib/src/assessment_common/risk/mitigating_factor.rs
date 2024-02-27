use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::subject_reference::SubjectReference,
    metadata::{Link, Property},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MitigatingFactor {
    pub uuid: UUIDDatatype,
    pub implementation_uuid: Option<UUIDDatatype>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub subjects: Option<Vec<SubjectReference>>,
}

impl SchemaConstraint for MitigatingFactor {
    fn constraint_title() -> &'static str {
        "Mitigating Factor"
    }
    fn constraint_description() -> &'static str {
        "Describes an existing mitigating factor that may affect the overall determination of the risk, with an optional link to an implementation statement in the SSP."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:mitigating-factor"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:mitigating-factor"
    }
}
