use keyring::{self, Entry};
use zeroize::Zeroizing;
use super::Error;

pub fn create(name: &str, value: Zeroizing<String>) -> Result<(), Error> {
    let entry = Entry::new("signet", name);
    entry.set_password(&value)?;
    Ok(())
}

pub fn lookup(name: &str) -> Result<Zeroizing<String>, Error> {
    let entry = Entry::new("signet", name);
    let value = entry.get_password()?;
    Ok(Zeroizing::new(value))
}

impl From<keyring::Error> for Error {
    fn from(error: keyring::Error) -> Self {
        Self::System(error.to_string())
    }
}
