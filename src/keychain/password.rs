use std::fmt;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{PasswordHash, SaltString};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;
use crate::{Secret, System};

#[derive(Debug, Deserialize, Serialize)]
pub enum Password {
    Secret(Secret),
    Static(Static),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Static {
    pub hash: String,
}

#[derive(Debug)]
pub enum Error {
    Crypto(String),
    System(String),
    Mismatch,
}

impl Password {
    pub fn create<S: System>(secret: bool) -> Result<Self, Error> {
        let password = request::<S>()?;
        Ok(match secret {
            true  => Self::Secret(Secret::create(password)?),
            false => Self::Static(Static::create(password)?),
        })
    }

    pub fn lookup<S: System>(&self) -> Result<Zeroizing<String>, Error> {
        Ok(match self {
            Self::Secret(p) => p.lookup()?,
            Self::Static(p) => p.lookup::<S>()?,
            Self::String(s) => Zeroizing::new(s.clone()),
        })
    }

    pub fn insecure(string: &str) -> Self {
        Self::String(string.to_owned())
    }
}

impl Static {
    fn create(password: Zeroizing<String>) -> Result<Self, Error> {
        let salt = SaltString::generate(OsRng);
        let hash = argon2(&password, &salt)?;
        let hash = hash.to_string();
        Ok(Self { hash })
    }

    fn lookup<S: System>(&self) -> Result<Zeroizing<String>, Error> {
        let password = S::prompt("password: ")?;
        let password = Zeroizing::new(password);

        let argon2 = Argon2::default();
        let hash   = PasswordHash::new(&self.hash)?;
        argon2.verify_password(password.as_bytes(), &hash)?;

        Ok(password)
    }
}

fn request<S: System>() -> Result<Zeroizing<String>, Error> {
    let password = Zeroizing::new(S::prompt("enter password: ")?);
    let repeated = Zeroizing::new(S::prompt("password again: ")?);
    match password == repeated {
        true  => Ok(password),
        false => Err(Error::Mismatch),
    }
}

fn argon2<'a>(password: &str, salt: &'a SaltString) -> Result<PasswordHash<'a>, Error> {
    let argon2 = Argon2::default();
    let bytes  = password.as_bytes();
    Ok(argon2.hash_password(bytes, salt)?)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Crypto(error) => write!(f, "{error}"),
            Self::System(error) => write!(f, "{error}"),
            Self::Mismatch      => write!(f, "password mismatch"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::System(error.to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(error: argon2::password_hash::Error) -> Self {
        Self::Crypto(error.to_string())
    }
}

impl From<crate::platform::Error> for Error {
    fn from(error: crate::platform::Error) -> Self {
        Self::System(error.to_string())
    }
}
