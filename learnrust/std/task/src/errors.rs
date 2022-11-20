use std::{ io, env };
use shellexpand::LookupError;
use uuid;

pub type Result<T> = std::result::Result<T, TaskError>;

pub enum TaskError {
    FailedToExpandPath(String, LookupError<env::VarError>),
    FailedToOpenFile(String, io::Error),
    FailedToReadLine(io::Error),
    BadLineFormat(String, String),
    BadPropertyFormat(String, String),
    UnknownKey(String),
    BadUuid(String, uuid::Error),
    UnquotedString(String),
    CharacterNotFound(String, char),
    EntryAlreadyParsed(String),
    FailedToParseDateTime(String),
    FailedToParseMask(String),
    EmptyString,
    FailedToParseRecur(String),
    FailedToParseStatus(String),
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::FailedToExpandPath(path, err) =>
                write!(f, "Failed to expand path `{}`: {}", path, err),
            TaskError::FailedToOpenFile(path, err) =>
                write!(f, "Failed to open file `{}`: {}", path, err),
            TaskError::FailedToReadLine(err) => write!(f, "Failed to read line: {}", err),
            TaskError::BadLineFormat(line, hint) =>
                write!(f, "Bad line format: {}, for line:\n{}", hint, line),
            TaskError::BadPropertyFormat(prop, hint) =>
                write!(f, "Bad property format: {}, for prop:\n{}", hint, prop),
            TaskError::UnknownKey(key) => write!(f, "Unknown key: {}", key),
            TaskError::BadUuid(uuid, err) => write!(f, "Bad UUID `{}`: {}", uuid, err),
            TaskError::UnquotedString(value) =>
                write!(f, "Expected a quoted string, got: {}", value),
            TaskError::CharacterNotFound(s, ch) =>
                write!(f, "Character `{}` not found in `{}`", ch, s),
            TaskError::EntryAlreadyParsed(entry) =>
                write!(f, "Entry `{}` is already parsed", entry),
            TaskError::FailedToParseDateTime(dt) => write!(f, "Failed to parse DateTime `{}`", dt),
            TaskError::FailedToParseMask(mask) =>
                write!(f, "Failed to parse recurrence mask `{}`", mask),
            TaskError::EmptyString => write!(f, "String value should not be empty"),
            TaskError::FailedToParseRecur(recur) =>
                write!(f, "Failed to parse recurrence periodicity `{}`", recur),
            TaskError::FailedToParseStatus(status) =>
                write!(f, "Failed to parse status `{}`", status),
        }
    }
}

impl std::fmt::Debug for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::error::Error for TaskError {}