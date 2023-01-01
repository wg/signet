use anyhow::Result;
use ssh_key::{HashAlg, SshSig};
use crate::{Signet, System};
use crate::args::Verify;
use crate::persist::read;

pub fn verify<S: System>(signet: &Signet<S>, args: Verify) -> Result<()> {
    let Verify { sig, ns, data } = args;

    let msg = read::<S>(&data)?;
    let sig = read::<S>(&sig.into())?;
    let sig = SshSig::from_pem(sig)?;

    let fp  = sig.public_key().fingerprint(HashAlg::Sha256);
    let key = hex::encode(fp);
    let key = signet.find(&key)?;

    key.public_key().verify(&ns, &msg, &sig)?;

    println!("good signature from {fp}");

    Ok(())
}
