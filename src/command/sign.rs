use std::path::{Path, PathBuf};
use anyhow::Result;
use ssh_key::{HashAlg, LineEnding};
use crate::{Input, Signet, System};
use crate::args::Sign;
use crate::persist::{read, Context};

pub fn sign<S: System>(signet: &Signet<S>, args: Sign) -> Result<()> {
    let Sign { key, ns, data } = args;

    let keychain = signet.current()?;
    let password = keychain.password();
    let key      = keychain.find(&key)?;
    let msg      = read::<S>(&data)?;
    let password = password.lookup::<S>()?;

    let key = key.decrypt(password)?;
    let sig = key.sign(&ns, HashAlg::Sha256, &msg)?;
    let sig = sig.to_pem(LineEnding::default())?;

    write::<S>(&data, sig.as_bytes())?;

    Ok(())
}

fn write<S: System>(input: &Input, data: &[u8]) -> Result<()> {
    let output = match input {
        Input::File(path) => append(path, ".sig"),
        Input::Stdin      => "/dev/stdout".into(),
    };
    Ok(S::write(&output, data).context(&output)?)
}

fn append(path: &Path, suffix: &str) -> PathBuf {
    let mut filename = path.file_name().unwrap_or_default().to_owned();
    filename.push(suffix);
    path.with_file_name(filename)
}
