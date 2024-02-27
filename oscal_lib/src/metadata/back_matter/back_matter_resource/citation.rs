use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    SchemaConstraint,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Citation {
    pub text: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
}
impl SchemaConstraint for Citation {
    fn constraint_title() -> &'static str {
        "Citation"
    }

    fn constraint_description() -> &'static str {
        "A citation consisting of end note text and optional structured bibliographic data."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter:resource:citation"
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/citation"
    }
}
