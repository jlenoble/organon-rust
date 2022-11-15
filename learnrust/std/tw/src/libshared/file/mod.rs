use std::fs;
use crate::{ AsPath, AsPathMut, Path, Result, TWError };

pub struct File {
    path: Path,
}

impl AsPath for File {
    fn as_path(&self) -> &Path {
        &self.path
    }

    fn as_str(&self) -> &str {
        self.path.as_str()
    }
}

impl AsPathMut for File {
    fn as_path_mut(&mut self) -> &mut Path {
        &mut self.path
    }
}

impl File {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            path: Path::new(path)?,
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Opens if necessary.
    pub fn read(&self) -> Result<String> {
        fs::read_to_string(self.path.as_str()).or(
            Err(TWError::FailedToReadFile(self.path.as_str().to_owned()))
        )
    }
}