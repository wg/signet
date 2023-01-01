use anyhow::Result;
use crate::{Signet, System};
use crate::args::Init;
use crate::keychain::Password;

pub fn init<S: System>(signet: &Signet<S>, args: Init) -> Result<()> {
    let password = Password::create::<S>(args.secret)?;
    signet.initialize(password)?;
    Ok(println!("signet initialized!"))
}
