use chrono::prelude::*;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::global::{Properties, VersionVar};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Revision {
    pub title: Option<String>,
    pub published: Option<DateTime<Utc>>,
    pub last_modified: Option<DateTime<Utc>>,
    pub version: VersionVar,
    pub oscal_version: Option<Version>,
    pub props: Option<Properties>,
    pub links: Option<Vec<String>>,
}

pub type Revisions = Vec<Revision>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let version = VersionVar::from("1.0.0");
        let r = Revision {
            title: None,
            published: None,
            last_modified: None,
            version,
            oscal_version: None,
            props: None,
            links: None,
        };
        let result = serde_json::to_string(&r);
        assert!(result.is_ok());
    }
}
