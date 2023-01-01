use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Keystore {
    version:   u64,
    keychains: Vec<String>,
    current:   Option<String>,
}

#[derive(Debug)]
pub enum Error {
    KeychainNotFound,
    KeychainNotSelected,
}

impl Keystore {
    pub fn insert(&mut self, name: &str) {
        self.keychains.push(name.to_owned());
    }

    pub fn current(&self) -> Result<&str, Error> {
        match &self.current {
            Some(name) => Ok(name),
            None       => Err(Error::KeychainNotSelected),
        }
    }

    pub fn select(&mut self, name: &str) -> Result<(), Error> {
        let name = self.keychains.iter().find(|k| {
            *k == name
        }).cloned().ok_or(Error::KeychainNotFound)?;
        self.current = Some(name);
        Ok(())
    }
}

impl Default for Keystore {
    fn default() -> Self {
        Self {
            version:   1,
            keychains: Vec::new(),
            current:   None,
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::KeychainNotFound    => write!(f, "keychain not found"),
            Self::KeychainNotSelected => write!(f, "no keychain selected"),
        }
    }
}
