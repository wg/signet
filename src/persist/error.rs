use std::fmt;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Error {
    cause: Cause,
    path:  PathBuf
}

#[derive(Debug)]
pub enum Cause {
    Invalid(serde_yaml::Error),
    Missing(io::Error),
    System(io::Error),
}

pub trait Context<T> {
    fn context(self, path: &Path) -> Result<T, Error>;
}

impl<T, E: Into<Cause>> Context<T> for Result<T, E> {
    fn context(self, path: &Path) -> Result<T, Error> {
        self.map_err(|error| Error {
            cause: error.into(),
            path:  path.to_owned(),
        })
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path = self.path.display();
        match &self.cause {
            Cause::Invalid(error) => write!(f, "{path}: {error}"),
            Cause::Missing(_)     => write!(f, "{path} not found"),
            Cause::System(error)  => write!(f, "{path}: {error}"),
        }
    }
}

impl From<io::Error> for Cause {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => Self::Missing(error),
            _                   => Self::System(error),
        }
    }
}

impl From<serde_yaml::Error> for Cause {
    fn from(error: serde_yaml::Error) -> Self {
        Self::Invalid(error)
    }
}
