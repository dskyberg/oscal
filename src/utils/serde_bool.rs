use serde::{de, Serializer};

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "yes" | "Yes" | "YES" => Ok(true),
        "no" | "No" | "NO" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["yes", "no"])),
    }
}

pub fn deserialize_option_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "yes" | "Yes" | "YES" => Ok(Some(true)),
        "no" | "No" | "NO" => Ok(Some(false)),
        _ => Err(de::Error::unknown_variant(s, &["yes", "no"])),
    }
}

pub fn serialize_bool<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        true => serializer.serialize_str("yes"),
        false => serializer.serialize_str("no"),
    }
}

pub fn serialize_option_bool<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(true) => serializer.serialize_str("yes"),
        Some(false) => serializer.serialize_str("no"),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Dummy {
        #[serde(deserialize_with = "deserialize_bool")]
        #[serde(serialize_with = "serialize_bool")]
        bool: bool,
    }

    #[test]
    fn test_serialize_include_control() {
        let c = Dummy { bool: true };
        let result = serde_json::to_string(&c).expect("oops");
        println!("{}", &result);
    }

    #[test]
    fn test_deserialize_include_control() {
        let json = r#"{"bool": "yes"}"#;
        let result = serde_json::from_str::<Dummy>(json);
        assert!(result.is_ok());
        assert!(result.unwrap().bool);

        let json = r#"{"bool": "Yes"}"#;
        let result = serde_json::from_str::<Dummy>(json);
        assert!(result.is_ok());
        assert!(result.unwrap().bool);

        let json = r#"{"bool": "YES"}"#;
        let result = serde_json::from_str::<Dummy>(json);
        assert!(result.is_ok());
        assert!(result.unwrap().bool);

        let json = r#"{"bool": "no"}"#;
        let result = serde_json::from_str::<Dummy>(json);
        assert!(result.is_ok());
        assert!(!result.unwrap().bool);

        let json = r#"{"bool": "No"}"#;
        let result = serde_json::from_str::<Dummy>(json);
        assert!(result.is_ok());
        assert!(!result.unwrap().bool);

        let json = r#"{"bool": "NO"}"#;
        let result = serde_json::from_str::<Dummy>(json);
        assert!(result.is_ok());
        assert!(!result.unwrap().bool);
    }
}
