use std::io::Result;
use std::path::{Path, PathBuf};

pub trait System {
    fn init(path: &Path, data: &[u8])  -> Result<()>;
    fn sync(path: &Path, data: &[u8])  -> Result<()>;

    fn mkdir(path: &Path)              -> Result<()>;
    fn read(input: &Input)             -> Result<Vec<u8>>;
    fn write(path: &Path, data: &[u8]) -> Result<()>;

    fn prompt(prompt: &str)            -> Result<String>;
}

#[derive(Clone, Debug)]
pub enum Input {
    File(PathBuf),
    Stdin,
}

impl From<PathBuf> for Input {
    fn from(path: PathBuf) -> Self {
        Self::File(path)
    }
}
