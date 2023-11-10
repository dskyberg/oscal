pub trait SchemaConstraint {
    fn constraint_title() -> &'static str;
    fn constraint_description() -> &'static str;
    fn constraint_id() -> &'static str;
    fn schema_path() -> &'static str;
}
