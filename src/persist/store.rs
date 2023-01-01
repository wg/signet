use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml::{from_slice, to_string};
use crate::{Keychain, Keystore, System};
use super::{Context, Error};

pub struct Store<S> {
    root: PathBuf,
    sys:  PhantomData<S>,
}

impl<S: System> Store<S> {
    pub fn new(root: PathBuf) -> Self {
        let sys = PhantomData;
        Self { root, sys }
    }

    pub fn initialize(&self) -> Result<(), Error> {
        let path = &self.root.join("chains");
        S::mkdir(path).context(path)
    }

    pub fn keystore(&self) -> Result<Keystore, Error> {
        self.load(&keystore())
    }

    pub fn keychain(&self, name: &str) -> Result<Keychain, Error> {
        self.load(&keychain(name))
    }

    pub fn init<T: Persistent>(&self, data: &T) -> Result<(), Error> {
        let path  = &self.root.join(data.filename());
        let bytes = to_string(data).context(path)?;
        S::init(path, bytes.as_bytes()).context(path)
    }

    pub fn sync<T: Persistent>(&self, data: &T) -> Result<(), Error> {
        let path  = &self.root.join(data.filename());
        let bytes = to_string(data).context(path)?;
        S::sync(path, bytes.as_bytes()).context(path)
    }

    fn load<T: DeserializeOwned>(&self, path: &Path) -> Result<T, Error> {
        let path  = &self.root.join(path);
        let input = path.clone().into();
        let bytes = S::read(&input).context(path)?;
        from_slice(&bytes).context(path)
    }
}

pub trait Persistent: DeserializeOwned + Serialize {
    fn filename(&self) -> PathBuf;
}

impl Persistent for Keychain {
    fn filename(&self) -> PathBuf {
        keychain(&self.metadata().identity)
    }
}

impl Persistent for Keystore {
    fn filename(&self) -> PathBuf {
        keystore()
    }
}

fn keystore() -> PathBuf {
    Path::new("signet").with_extension("yml")
}

fn keychain(name: &str) -> PathBuf {
    Path::new("chains").join(name).with_extension("yml")
}
