use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractionError {
    // Too many segments
    #[error("expected only one segment")]
    NotExactlyOneSegment,

    // Not a bare identifier segment
    #[error("found parenthesized arguments in segment, as the (A, B) -> C in Fn(A, B) -> C.")]
    ParenthesizedArguments,

    // Not a bare identifier path
    #[error("found leading double colon in path")]
    LeadingDoubleColon,
    #[error("path should not be qualified, as in <Self as HasItem>::Item")]
    QualifiedPath,

    // Not a named field declaring a type
    #[error("field should be named")]
    UnnamedField,
}