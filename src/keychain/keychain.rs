use std::fmt;
use std::ops::Deref;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use ssh_key::{HashAlg, PrivateKey};
use super::{Password, Provider};

#[derive(Debug, Deserialize, Serialize)]
pub struct Keychain {
    keychain: IndexMap<String, Key>,
    metadata: Metadata,
    provider: Provider,
}

#[derive(Clone, Debug)]
pub enum Key {
    SSH(PrivateKey),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub identity: String,
    pub password: Password,
}

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    KeyAmbiguous,
}

impl Keychain {
    pub fn new(metadata: Metadata, provider: Provider) -> Self {
        Self {
            keychain: IndexMap::new(),
            metadata: metadata,
            provider: provider,
        }
    }

    pub fn add(&mut self, key: impl Into<Key>) -> String {
        let key = key.into();
        let id  = key.id();
        self.keychain.insert(id.clone(), key);
        id
    }

    pub fn delete(&mut self, key: &str) -> Option<Key> {
        self.keychain.remove(key)
    }

    pub fn find(&self, prefix: &str) -> Result<&Key, Error> {
        let keys = self.keychain.iter().filter_map(|(id, key)| {
            id.starts_with(prefix).then_some(key)
        }).collect::<Vec<_>>();

        match keys[..] {
            [key] => Ok(key),
            []    => Err(Error::KeyNotFound),
            [..]  => Err(Error::KeyAmbiguous),
        }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn password(&self) -> &Password {
        &self.metadata.password
    }

    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.keychain.values()
    }
}

impl Key {
    pub fn id(&self) -> String {
        let Self::SSH(key) = self;
        let algorithm   = HashAlg::Sha256;
        let fingerprint = key.fingerprint(algorithm);
        hex::encode(fingerprint)
    }
}

impl Deref for Key {
    type Target = PrivateKey;

    fn deref(&self) -> &Self::Target {
        let Key::SSH(ref key) = self;
        key
    }
}

impl From<PrivateKey> for Key {
    fn from(key: PrivateKey) -> Self {
        Self::SSH(key)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::KeyNotFound   => write!(f, "key not found"),
            Self::KeyAmbiguous  => write!(f, "key ambiguous"),
        }
    }
}
