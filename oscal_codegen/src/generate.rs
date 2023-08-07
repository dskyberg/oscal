use crate::Result;
pub trait Generate {
    fn generate(&self, path: &str) -> Result<String>;
}
