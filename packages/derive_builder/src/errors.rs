use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractionError {
    // Too many nested
    #[error("expected only one nested meta")]
    NotExactlyOneNested,

    // Not a named field declaring a type
    #[error("field should be named")]
    UnnamedField,

    // Not a name-value meta
    #[error("nested meta should be `name = value`")]
    NotNameValueMeta,
    #[error("value in nested meta should be a string")]
    NotAString,
    #[error("expected meta as inert_attr(name = value)")]
    NotAMetaList,
}