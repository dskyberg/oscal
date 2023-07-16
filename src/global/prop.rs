use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prop {
    name: String,
    uuid: Option<Uuid>,
    ns: Option<String>, // TODO: turn into a valid Uri, or add validation
    value: String,
    ///A textual label that provides a sub-type or characterization of the property's name
    class: Option<String>, // TODO: validation pattern: "^(\\p{L}|_)(\\p{L}|\\p{N}|[.\\-_])*$"
    remarks: Option<String>, // TODO: Multi-line markdown validation
}

pub type Props = Vec<Prop>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let uuid = Some(Uuid::new_v4());
        let name = "my-name".to_string();
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
        let prop = Prop {
            uuid,
            name,
            value,
            ns,
            class,
            remarks,
        };

        let json = serde_json::to_string(&prop);
        assert!(json.is_ok());
        let result = serde_json::from_str::<Prop>(&json.unwrap());
        assert!(result.is_ok());
    }
}
