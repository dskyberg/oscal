/// Extend schema elements
use serde_json::Value;

pub trait Extensible {
    fn extend_schema(&mut self, value: &Value) -> crate::error::Result<()>;
}
