/// Base64Datatype
/// 
/// $id: #/definitions/base_64_datatype

use serde::{de, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
#[serde(transparent)]
pub struct Base64Datatype {
#[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
	pub inner: String
}

impl Base64Datatype {
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


impl TryFrom<&str> for Base64Datatype {
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
    let s: String = de::Deserialize::deserialize(deserializer)?;

    match Base64Datatype::validate(&s) {
        Ok(()) => Ok(s),
        _ => Err(de::Error::custom("invalid Base64Datatype")),
    }
}

fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value)
}
