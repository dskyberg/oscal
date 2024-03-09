pub trait SchemaElement {
    /// The schema object `title` attribute
    fn schema_title() -> &'static str;
    /// The schema object `description` attribute.
    fn schema_description() -> &'static str;
    /// The schema object `$id` attribute.
    /// If the object does not have an `$id` attribute, `None` is returned.
    fn schema_id() -> Option<&'static str>;
    /// The JSON path to the schema object.
    /// This should always start with `"#/definitions/`.
    fn schema_path() -> &'static str;
}
