/// NonNegativeIntegerDatatype
/// 
/// $id: #/definitions/non_negative_integer_datatype

use serde::{de, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
#[serde(transparent)]
pub struct NonNegativeIntegerDatatype {
#[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
	pub inner: i64
}

impl NonNegativeIntegerDatatype {
    pub fn new(value: i64) -> Self {
        Self{inner: value}
    }

    /// String references are established so that the value can be
    /// validated against a pattern or other connstraint.
    pub fn validate(_value: i64) -> crate::error::Result<()> {
        //todo!(); // Replace with appropriate validation method.
        Ok(())
    }
}


impl TryFrom<i64> for NonNegativeIntegerDatatype {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match Self::validate(value) {
            Ok(()) => Ok(Self::new(value)),
            Err(e) => Err(e),
        }
    }
}

fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: i64 = de::Deserialize::deserialize(deserializer)?;

    match NonNegativeIntegerDatatype::validate(s) {
        Ok(()) => Ok(s),
        _ => Err(de::Error::custom("not a number"))
    }
}

fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(*value)
}
