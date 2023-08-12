use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Object is a type {0}")]
    WrongObjectType(String),
    #[error("Bad schema")]
    BadSchema,
    #[error("String type expected")]
    StringExpected,
    #[error("Object type expected")]
    ObjectExpected,
    #[error("Array type expected")]
    ArrayExpected,
    #[error("Number type expected")]
    NumberExpected,
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Stacked error: {0}")]
    StackedError(Box<dyn std::error::Error>),
    #[error("Error parsing EnumeratedType")]
    BadEnumeratedType,
    #[error("Failed parsing PropertyType")]
    BadPropertyType,
    #[error("Unrecognized type: {0}")]
    UnrecognizedType(String),
    #[error("The value is not an embedded namespace: {0}")]
    NotEmbeddedId(String),
    #[error("The value is not a uri style id")]
    NotUriId(String),
    #[error("Don't know how to merge the ID")]
    MergeFailed,
    #[error("Expected this node to be terminal")]
    ExpectedTerminalNode,
    #[error("Cannot be generated")]
    CannotGenerate,
    #[error("Wrong property type for NameSpace")]
    WrongType,
    #[error("Property type is not extensible")]
    NotExstensible,
    #[error("Bad extension object: {0}")]
    BadExtension(String),
}
