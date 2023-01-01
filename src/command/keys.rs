use anyhow::Result;
use rand_core::OsRng;
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use crate::{Input, Signet, System};
use crate::args::Keys;
use crate::persist::read;

pub fn keys<S: System>(signet: &Signet<S>, cmd: Keys) -> Result<()> {
    match cmd {
        Keys::Create      => create(signet),
        Keys::Delete(key) => delete(signet, &key),
        Keys::Export(key) => export(signet, &key),
        Keys::Import(key) => import(signet, &key),
        Keys::Public(key) => public(signet, &key),
        Keys::List        => list(signet),
    }
}

pub fn create<S: System>(signet: &Signet<S>) -> Result<()> {
    let mut keychain = signet.current()?;

    let password = keychain.password();
    let password = password.lookup::<S>()?;

    let alg = Algorithm::Ed25519;
    let key = PrivateKey::random(OsRng, alg)?;

    let key = key.encrypt(OsRng, password)?;
    let id  = keychain.add(key);
    signet.sync(&keychain)?;

    Ok(println!("created key {id}"))
}

pub fn delete<S: System>(signet: &Signet<S>, key: &str) -> Result<()> {
    let mut keychain = signet.current()?;

    let key = keychain.find(key)?;
    let id  = key.id();

    keychain.delete(&id);
    signet.sync(&keychain)?;

    Ok(println!("deleted key {id}"))
}

pub fn export<S: System>(signet: &Signet<S>, key: &str) -> Result<()> {
    let keychain = signet.current()?;
    let password = keychain.password();
    let password = password.lookup::<S>()?;

    let key = keychain.find(key)?;
    let key = key.decrypt(password)?;

    let password = S::prompt("export password: ")?;
    let exported = key.encrypt(OsRng, password)?;
    let exported = &*exported.to_openssh(LineEnding::default())?;

    Ok(print!("{exported}"))
}

pub fn import<S: System>(signet: &Signet<S>, key: &Input) -> Result<()> {
    let mut keychain = signet.current()?;

    let key = read::<S>(key)?;
    let key = PrivateKey::from_openssh(key)?;

    let password = keychain.password();
    let password = password.lookup::<S>()?;

    let key = match key.is_encrypted() {
        true  => decrypt::<S>(key)?,
        false => key,
    }.encrypt(OsRng, password)?;

    let id = keychain.add(key);
    signet.sync(&keychain)?;

    Ok(println!("imported key {id}"))
}

pub fn public<S: System>(signet: &Signet<S>, key: &str) -> Result<()> {
    let keychain = signet.current()?;

    let key = keychain.find(key)?;
    let key = key.public_key().to_string();

    Ok(println!("{key}"))
}

pub fn list<S: System>(signet: &Signet<S>) -> Result<()> {
    let keystore = signet.keystore()?;
    let current  = keystore.current()?;
    let keychain = signet.keychain(current)?;

    let list = keychain.keys().map(|key| {
        format!("{:>66}", key.id())
    }).collect::<Vec<_>>().join("\n");

    Ok(println!("keychain '{current}':\n{list}"))
}

fn decrypt<S: System>(key: PrivateKey) -> Result<PrivateKey> {
    let password = S::prompt("key password: ")?;
    Ok(key.decrypt(password)?)
}
