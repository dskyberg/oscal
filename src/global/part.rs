use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::{Links, NCName, Properties};

// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub id: Option<NCName>,
    pub name: NCName,
    pub ns: Option<String>,
    pub class: Option<NCName>,
    pub title: Option<String>,
    pub props: Option<Properties>,
    pub prose: Option<String>,
    pub links: Option<Links>,
    pub parts: Option<Parts>,
}

pub type Parts = Vec<Part>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = include_str!("../../tests/parts.json");
        let result: Part = serde_json::from_str(json).expect("failed de");
        dbg!(&result);
    }
}
