pub type Result<T> = std::result::Result<T, TWError>;

pub enum TWError {
    MissingEnvVariable(String),
    FailedToExpandPath(String),
    BadConfigEntry(String),
}

impl std::fmt::Display for TWError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TWError::MissingEnvVariable(var) => {
                write!(f, "Environment variable `{}` is undefined", var)
            }
            TWError::FailedToExpandPath(path) => { write!(f, "Failed to expand path `{}`", path) }
            TWError::BadConfigEntry(entry) => {
                write!(f, "Malformed entry `{}` in config file", entry)
            }
        }
    }
}

impl std::fmt::Debug for TWError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::error::Error for TWError {}