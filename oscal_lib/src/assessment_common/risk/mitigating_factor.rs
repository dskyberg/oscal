use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::subject_reference::SubjectReference,
    metadata::{Link, Property},
    SchemaElement, UUIDDatatype,
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

impl SchemaElement for MitigatingFactor {
    fn schema_title() -> &'static str {
        "Mitigating Factor"
    }
    fn schema_description() -> &'static str {
        "Describes an existing mitigating factor that may affect the overall determination of the risk, with an optional link to an implementation statement in the SSP."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:mitigating-factor"
    }
}
