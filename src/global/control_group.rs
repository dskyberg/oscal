use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Controls, Parts};
use crate::global::{Links, Parameters, Properties, Token};
// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlGroup {
    pub id: Option<Token>,
    pub class: Option<Token>,
    pub title: String,
    pub params: Option<Parameters>,
    pub props: Option<Properties>,
    pub links: Option<Links>,
    pub parts: Option<Parts>,
    pub groups: Option<ControlGroups>,
    pub controls: Option<Controls>,
}

pub type ControlGroups = Vec<ControlGroup>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!("../../tests/group_1.json");
        let result = serde_json::from_str::<ControlGroup>(json);
        assert!(result.is_ok());
    }
}
