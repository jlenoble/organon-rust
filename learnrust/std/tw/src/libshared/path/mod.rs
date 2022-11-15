use shellexpand;
use crate::{ Result, TWError };

pub struct Path {
    _original: String,
    _data: String,
}

pub trait AsPath {
    fn as_path(&self) -> &Path;
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

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self._data).exists()
    }
}