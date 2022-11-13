use crate::{ AsPath, AsPathMut, Path, Result };

pub struct File {
    path: Path,
}

impl AsPath for File {
    fn as_path(&self) -> &Path {
        &self.path
    }
}

impl AsPathMut for File {
    fn as_path_mut(&mut self) -> &mut Path {
        &mut self.path
    }
}

impl File {
    pub fn new(path: &'static str) -> Result<Self> {
        Ok(Self {
            path: Path::new(path)?,
        })
    }
}