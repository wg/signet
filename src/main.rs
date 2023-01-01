use anyhow::Result;
use signet::signet;
use signet::args::{args, Command};
use signet::command::{init, keys, sign, verify};

fn main() -> Result<()> {
    let (root, command) = args()?;
    let signet = signet(root);

    match command {
        Command::Init(cmd)   => init(&signet, cmd)?,
        Command::Keys(cmd)   => keys(&signet, cmd)?,
        Command::Sign(cmd)   => sign(&signet, cmd)?,
        Command::Verify(cmd) => verify(&signet, cmd)?,
        Command::Compat      => println!("COMPAT"),
    };

    Ok(())
}
