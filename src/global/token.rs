use serde::{Deserialize, Serialize};

use crate::OscalError;

/// An XML non-colonized name value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Token {
    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    pub inner: String,
}

impl Token {
    fn new(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
    /// Returns `true` if `c` is a legal `NCNameStartChar` as defined
    /// by the ["Namespaces in XML 1.1"
    /// specification](https://www.w3.org/TR/xml-names11/#NT-NameStartChar).
    fn is_valid_start_char(c: char) -> bool {
        matches!(c,
            'a'..='z'|
            'A'..='Z'|
            '_'|
            '\u{C0}'..='\u{D6}'|
            '\u{D8}'..='\u{F6}'|
            '\u{F8}'..='\u{2FF}'|
            '\u{370}'..='\u{37D}'|
            '\u{37F}'..='\u{1FFF}'|
            '\u{200C}'..='\u{200D}'|
            '\u{2070}'..='\u{218F}'|
            '\u{2C00}'..='\u{2FEF}'|
            '\u{3001}'..='\u{D7FF}'|
            '\u{F900}'..='\u{FDCF}'|
            '\u{FDF0}'..='\u{FFFD}'|
            '\u{10000}'..='\u{EFFFF}'
        )
    }

    /// Returns `true` if `c` is a legal `NCNameChar` as defined by
    /// the ["Namespaces in XML 1.1"
    /// specification](https://www.w3.org/TR/xml11/#NT-NameChar).
    fn is_valid_char(c: char) -> bool {
        matches!(c,
            'a'..='z'|
            'A'..='Z'|
            '_' | '-' | '.' | '\u{B7}'|
            '0'..='9'|
            '\u{C0}'..='\u{D6}'|
            '\u{D8}'..='\u{F6}'|
            '\u{F8}'..='\u{2FF}'|
            '\u{300}'..='\u{37D}'|
            '\u{37F}'..='\u{1FFF}'|
            '\u{200C}'..='\u{200D}'|
            '\u{203F}'..='\u{2040}'|
            '\u{2070}'..='\u{218F}'|
            '\u{2C00}'..='\u{2FEF}'|
            '\u{3001}'..='\u{D7FF}'|
            '\u{F900}'..='\u{FDCF}'|
            '\u{FDF0}'..='\u{FFFD}'|
            '\u{10000}'..='\u{EFFFF}'
        )
    }

    pub fn validate(token: &str) -> crate::Result<()> {
        let ts = token.chars();
        for (idx, c) in ts.enumerate() {
            match idx {
                0 => {
                    if idx == 0 && !Token::is_valid_start_char(c) {
                        println!("Bad first char: {}", c);
                        return Err(
                            OscalError::BadNCName(format!("illegal starting char: {}", c)).into(),
                        );
                    }
                }
                _ => {
                    if !Token::is_valid_char(c) {
                        return Err(OscalError::BadNCName(format!("illegal char: {}", c)).into());
                    }
                }
            }
        }

        Ok(())
    }
}

impl TryFrom<&str> for Token {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Token::validate(value) {
            Ok(()) => Ok(Token::new(value)),
            Err(e) => Err(e),
        }
    }
}

use serde::{de, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match Token::validate(s) {
        Ok(()) => Ok(s.to_string()),
        _ => Err(de::Error::unknown_variant(s, &["Token"])),
    }
}

pub fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Outer {
        pub id: Token,
    }

    #[test]
    fn test_from_str() {
        let result = Token::try_from("goodValue");
        assert!(result.is_ok());

        let result = Token::try_from("0_bad_value");
        assert!(result.is_err());
    }

    #[test]
    fn test_se() {
        let token: Token = Token::new("good_value");
        let outer = Outer { id: token };
        let result = serde_json::to_string(&outer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_de() {
        let json = r#"{"id":"good_value"}"#;
        let result = serde_json::from_str::<Outer>(json);
        assert!(result.is_ok());

        let json = r#"{"id":"0_bad_value"}"#;
        let result = serde_json::from_str::<Outer>(json);
        assert!(result.is_err());
    }
}
