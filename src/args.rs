use std::convert::Infallible;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::{anyhow, Result};
use bpaf::*;
use dirs::{config_dir, home_dir};
use crate::Input;

#[derive(Clone, Debug)]
pub enum Command {
    Init(Init),
    Keys(Keys),
    Sign(Sign),
    Verify(Verify),
    Compat,
}

#[derive(Clone, Debug)]
pub struct Init {
    pub secret: bool,
}

#[derive(Clone, Debug)]
pub enum Keys {
    Create,
    Delete(String),
    Export(String),
    Import(Input),
    Public(String),
    List,
}

#[derive(Clone, Debug)]
pub struct Sign {
    pub key:  String,
    pub ns:   String,
    pub data: Input,
}

#[derive(Clone, Debug)]
pub struct Verify {
    pub sig:  PathBuf,
    pub ns:   String,
    pub data: Input,
}

#[derive(Debug)]
pub struct Args {
    pub store:   PathBuf,
    pub command: Command
}

pub fn args() -> Result<(PathBuf, Command)> {
    let Args {
        store,
        command,
        ..
    } = parser().run();
    Ok((store, command))
}

pub fn parser() -> OptionParser<Args> {
    let store  = short('S').argument("DIR").hide();
    let store  = store.fallback_with(store_dir);

    let sig    = short('s').long("signature").argument("FILE");
    let ns     = short('n').long("namespace").argument("NAMESPACE");
    let data   = input("FILE");
    let verify = construct!(Verify { sig, ns, data });
    let verify = construct!(Command::Verify(verify)).to_options();

    let init   = init().command("init");
    let keys   = keys().command("keys");
    let sign   = sign().command("sign");
    let verify = verify.command("verify");
    let compat = compat();

    let command = construct!([init, keys, sign, verify, compat]);

    construct!(Args { store, command }).to_options()
}

fn init() -> OptionParser<Command> {
    let secret = short('s').long("secret").switch();
    let init   = construct!(Init { secret });
    construct!(Command::Init(init)).to_options()
}

fn keys() -> OptionParser<Command> {
    let create = short('c').long("create");
    let delete = short('d').long("delete").argument("KEY");
    let export = short('e').long("export").argument("KEY");
    let import = short('i').long("import").argument("FILE");
    let public = short('p').long("public").argument("KEY");

    let create = create.req_flag(Keys::Create);
    let delete = delete.map(Keys::Delete);
    let export = export.map(Keys::Export);
    let import = import.map(Keys::Import);
    let public = public.map(Keys::Public);

    let keys = construct!([
        create,
        delete,
        export,
        import,
        public,
    ]).fallback(Keys::List);

    construct!(Command::Keys(keys)).to_options()
}

fn sign() -> OptionParser<Command> {
    let key  = short('k').long("key").argument("KEY");
    let ns   = short('n').long("namespace").argument("NAMESPACE");
    let data = input("FILE");
    let sign = construct!(Sign { key, ns, data });
    construct!(Command::Sign(sign)).to_options()
}

fn input(name: &'static str) -> impl Parser<Input> {
    positional::<PathBuf>(name).optional().map(|path| {
        match path {
            Some(path) => Input::File(path),
            None       => Input::Stdin,
        }
    })
}

fn compat() -> impl Parser<Command> {
    let flag   = short('Y').switch().hide();
    let opts   = short('O').argument::<String>("").hide();
    let opts   = opts.many().anywhere();

    let key    = short('f').argument::<String>("").hide();
    let ns     = short('n').argument::<String>("").hide();
    let file   = input("FILE");
    let sign   = construct!(key, ns, file);
    let sign   = sign.parse(|(key, ns, data)| {
        Ok::<_, String>(Command::Sign(Sign { key, ns, data }))
    }).to_options().command("sign").hide();

    let file   = short('f').argument::<Input>("").hide();
    let id     = short('I').argument::<String>("").hide();
    let ns     = short('n').argument::<String>("").hide();
    let sig    = short('s').argument::<PathBuf>("").hide();
    let rev    = short('r').argument::<PathBuf>("").optional().hide();
    let verify = construct!(file, id, ns, sig, rev);
    let verify = verify.parse(|(_, _, ns, sig, _)| {
        let data = Input::Stdin;
        Ok::<_, String>(Command::Verify(Verify { sig, ns, data }))
    }).to_options().command("verify").hide();

    let file   = short('f').argument::<PathBuf>("").hide();
    let sig    = short('s').argument::<PathBuf>("").hide();
    let find   = construct!(file, sig);
    let find   = find.parse(|(_, _)| {
        Ok::<_, String>(Command::Compat)
    }).to_options().command("find-principals").hide();

    let ns     = short('n').argument::<String>("").hide();
    let sig    = short('s').argument::<PathBuf>("").hide();
    let check  = construct!(ns, sig);
    let check  = check.parse(|(ns, sig)| {
        let data = Input::Stdin;
        Ok::<_, String>(Command::Verify(Verify { sig, ns, data }))
    }).to_options().command("check-novalidate").hide();

    let command = construct!([sign, verify, check, find]);

    construct!(flag, opts, command).parse(|(_, _, cmd)| Ok::<_, String>(cmd))
}

fn store_dir() -> Result<PathBuf> {
    let root = match cfg!(target_os = "macos") {
        true  => home_dir().map(|home| home.join(".config")),
        false => config_dir(),
    }.ok_or_else(|| anyhow!("cannot determine config dir"))?;
    Ok(root.join("signet"))
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(Self::File(path.parse()?))
    }
}
