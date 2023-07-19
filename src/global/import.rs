/// The import designates a catalog or profile to be
/// included (referenced and potentially modified) by
/// this profile. The import also identifies which controls
/// to select using the include-all, include-controls, and
/// exclude-controls directives.
///
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::utils::{deserialize_option_bool, serialize_option_bool};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matching {
    pattern: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludeAllControlsConstraint {}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludeControlConstraint {
    /// When a control is included, whether its child (dependent) controls are also included.
    #[serde(rename = "with-child-controls")]
    #[serde(deserialize_with = "deserialize_option_bool")]
    #[serde(serialize_with = "serialize_option_bool")]
    with_child_controls: Option<bool>,
    #[serde(rename = "with-ids")]
    with_ids: Option<Vec<String>>,
    /// TODO: validate as token
    /// Select controls by (regular expression) match on ID
    matching: Option<Vec<Matching>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub href: String, // TODO: validate as Uri
    #[serde(rename = "include-all")]
    pub include_all: Option<IncludeAllControlsConstraint>,
    #[serde(rename = "include-controls")]
    pub include_controls: Option<IncludeControlConstraint>,
    #[serde(rename = "exclude-controls")]
    pub exclude_controls: Option<IncludeControlConstraint>,
}

pub type Imports = Vec<Import>;

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize, Deserialize)]
    struct Inner {}
    #[derive(Serialize, Deserialize)]
    enum Dummy {
        #[serde(rename = "first")]
        First {},

        Second {
            thing: String,
        },
    }

    #[test]
    fn test_it() {
        let dummy = Dummy::First {};

        let result = serde_json::to_string(&dummy).expect("oos");
        println!("{}", &result);
    }
}
