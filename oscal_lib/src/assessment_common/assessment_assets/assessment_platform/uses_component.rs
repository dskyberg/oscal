use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleParty},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UsesComponent {
    pub component_uuid: UUIDDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for UsesComponent {
    fn constraint_title() -> &'static str {
        "Uses Component"
    }
    fn constraint_description() -> &'static str {
        "The set of components that are used by the assessment platform."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-assets:assessment-platforrm:uses-component"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-assets:assessment-platforrm:uses-component"
    }
}
