use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{control::Controls, part::Parts};
use crate::global::{Links, Params, Props};
// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<String>,
    pub class: Option<String>,
    pub title: String,
    pub params: Option<Params>,
    pub props: Option<Props>,
    pub links: Option<Links>,
    pub parts: Option<Parts>,
    pub groups: Option<Groups>,
    pub controls: Option<Controls>,
}

pub type Groups = Vec<Group>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!("../../tests/group_1.json");
        let result = serde_json::from_str::<Group>(json);
        assert!(result.is_ok());
    }
}
