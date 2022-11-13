pub type Result = std::result::Result<(), TWError>;

pub enum TWError {
    MissingEnvVariable(&'static str),
}

impl std::fmt::Display for TWError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TWError::MissingEnvVariable(var) => {
                write!(f, "Environment variable `{}` is undefined", var)
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