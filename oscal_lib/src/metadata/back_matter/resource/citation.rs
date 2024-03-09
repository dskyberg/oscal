use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Citation {
    pub text: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
}
impl SchemaElement for Citation {
    fn schema_title() -> &'static str {
        "Citation"
    }

    fn schema_description() -> &'static str {
        "A citation consisting of end note text and optional structured bibliographic data."
    }

    fn schema_id() -> Option<&'static str> {
        None
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/citation"
    }
}
