use permissions;
use shellexpand;
use crate::{ Result, TWError };

pub struct Path {
    _original: String,
    _data: String,
}

pub trait AsPath {
    fn as_path(&self) -> &Path;
    fn as_str(&self) -> &str;
}

pub trait AsPathMut {
    fn as_path_mut(&mut self) -> &mut Path;
}

impl Path {
    pub fn new(path: &str) -> Result<Self> {
        let data = match shellexpand::full(path) {
            Ok(data) => data,
            Err(_) => {
                return Err(TWError::FailedToExpandPath(path.to_owned()));
            }
        };

        Ok(Self {
            _original: path.into(),
            _data: data.into(),
        })
    }

    pub fn as_str(&self) -> &str {
        self._data.as_str()
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self._data).exists()
    }

    ////////////////////////////////////////////////////////////////////////////////
    // EACCES is a permissions problem which is exactly what this method is trying
    // to determine.
    pub fn readable(&self) -> Result<bool> {
        permissions
            ::is_readable(self.as_str())
            .or(Err(TWError::FailedToReadFile(self.as_str().to_owned())))
    }
}