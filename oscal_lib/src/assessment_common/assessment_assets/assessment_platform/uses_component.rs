use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleParty},
    SchemaElement, UUIDDatatype,
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

impl SchemaElement for UsesComponent {
    fn schema_title() -> &'static str {
        "Uses Component"
    }
    fn schema_description() -> &'static str {
        "The set of components that are used by the assessment platform."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-assets:assessment-platforrm:uses-component"
    }
}
