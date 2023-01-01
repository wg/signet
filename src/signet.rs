use std::path::PathBuf;
use anyhow::Error;
use crate::{Key, Keychain, Keystore, Metadata, Password, Provider};
use crate::persist::{Persistent, Store};
use crate::system::System;

pub struct Signet<S> {
    store: Store<S>,
}

impl<S: System> Signet<S> {
    pub fn new(root: PathBuf) -> Self {
        let store = Store::new(root);
        Self { store }
    }

    pub fn initialize(&self, password: Password) -> Result<(), Error> {
        self.store.initialize()?;

        let default  = "default";
        let keychain = Keychain::new(Metadata {
            identity: default.to_owned(),
            password: password,
        }, Provider::Local);

        let mut keystore = Keystore::default();
        keystore.insert(default);
        keystore.select(default)?;

        self.store.init(&keystore)?;
        self.store.init(&keychain)?;

        Ok(())
    }

    pub fn current(&self) -> Result<Keychain, Error> {
        let keystore = self.keystore()?;
        let current  = keystore.current()?;
        self.keychain(current)
    }

    pub fn find(&self, id: &str) -> Result<Key, Error> {
        let keychain = self.current()?;
        Ok(keychain.find(id)?.clone())
    }

    pub fn keystore(&self) -> Result<Keystore, Error> {
        Ok(self.store.keystore()?)
    }

    pub fn keychain(&self, name: &str) -> Result<Keychain, Error> {
        Ok(self.store.keychain(name)?)
    }

    pub fn sync<T: Persistent>(&self, data: &T) -> Result<(), Error> {
        Ok(self.store.sync(data)?)
    }
}
