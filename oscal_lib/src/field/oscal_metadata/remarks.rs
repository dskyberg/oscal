/// Remarks
/// Additional commentary on the containing object.
/// $id: #field_oscal-metadata_remarks

use serde::{de, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
#[serde(transparent)]
pub struct Remarks {
#[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
	pub inner: String
}

impl Remarks {
    pub fn new(value: &str) -> Self {
        Self{inner: value.to_string()}
    }

    /// String references are established so that the value can be
    /// validated against a pattern or other connstraint.
    pub fn validate(_value: &str) -> crate::error::Result<()> {
        // todo!(); // Replace with appropriate validation method.
        Ok(())
    }
}


impl TryFrom<&str> for Remarks {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Self::validate(value) {
            Ok(()) => Ok(Self::new(value)),
            Err(e) => Err(e),
        }
    }
}

fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match Remarks::validate(s) {
        Ok(()) => Ok(s.to_string()),
        _ => Err(de::Error::unknown_variant(s, &["Remarks"])),
    }
}

fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value)
}