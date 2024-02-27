use crate::Error;

pub trait SchemaConstraint {
    fn constraint_title() -> &'static str;
    fn constraint_description() -> &'static str;
    fn constraint_id() -> &'static str;
    fn schema_path() -> &'static str;
    /// Test the value to determine whether it is valid
    fn validate(_value: &str) -> Result<(), Error> {
        Ok(())
    }
}
