/// Variable Version
///
/// Version data can be formatted as eitther
///
/// - Semantic Version (per RFC-)
/// - DateTime with TZ (RFC 3339)
/// - Generic string
///
use chrono::prelude::*;
use semver::Version;
use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum VersionVar {
    /// Version is in SemVer format
    Semver(Version),
    /// Version is in DateTime format
    Date(DateTime<Utc>),
    /// Version is in String format
    String(String),
}

impl From<&str> for VersionVar {
    fn from(value: &str) -> Self {
        // See if this is a valid SemVer
        if let Ok(s) = serde_json::from_str::<Version>(value) {
            return VersionVar::Semver(s);
        }

        if let Ok(d) = serde_json::from_str::<DateTime<Utc>>(value) {
            return VersionVar::Date(d);
        }

        VersionVar::String(value.to_string())
    }
}

impl From<Version> for VersionVar {
    fn from(value: Version) -> Self {
        VersionVar::Semver(value)
    }
}

impl From<DateTime<Utc>> for VersionVar {
    fn from(value: DateTime<Utc>) -> Self {
        VersionVar::Date(value)
    }
}

/// Version is always serialized as a String.  Therefor, we need to
/// determine the actual format when deserializing.
impl<'de> Visitor<'de> for VersionVar {
    type Value = Self;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E>(self, value: &str) -> Result<VersionVar, E>
    where
        E: serde::de::Error,
    {
        Ok(VersionVar::from(value))
    }
}
