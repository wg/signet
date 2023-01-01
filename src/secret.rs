use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;
use crate::platform::Error;
use crate::platform::secret::{create, lookup};

#[derive(Debug, Deserialize, Serialize)]
pub struct Secret {
    pub name: String,
}

impl Secret {
    pub fn create(value: Zeroizing<String>) -> Result<Self, Error> {
        let mut name = [0u8; 16];
        OsRng.fill_bytes(&mut name);

        let name = hex::encode(name);
        create(&name, value)?;

        Ok(Self { name })
    }

    pub fn lookup(&self) -> Result<Zeroizing<String>, Error> {
        lookup(&self.name)
    }
}
