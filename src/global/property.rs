use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::global::NCName;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: NCName,
    pub uuid: Option<Uuid>,
    pub ns: Option<String>, // TODO: turn into a valid Uri, or add validation
    pub value: String,
    ///A textual label that provides a sub-type or characterization of the property's name
    pub class: Option<NCName>, // TODO: validation pattern: "^(\\p{L}|_)(\\p{L}|\\p{N}|[.\\-_])*$"
    pub remarks: Option<String>, // TODO: Multi-line markdown validation
}

pub type Properties = Vec<Property>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let uuid = Some(Uuid::new_v4());
        let name = NCName::try_from("my-name").expect("oops");
        let value = "my value".to_string();
        let ns = None;
        let class = None;
        let remarks = Some(
            r#"
        Here's a typical line.

        And here is another.
        "#
            .to_string(),
        );
        let prop = Property {
            uuid,
            name,
            value,
            ns,
            class,
            remarks,
        };

        let json = serde_json::to_string(&prop);
        assert!(json.is_ok());
        let result = serde_json::from_str::<Property>(&json.unwrap());
        assert!(result.is_ok());
    }
}
