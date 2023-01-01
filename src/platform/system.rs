use std::fs::{self, File, OpenOptions};
use std::io::{stdin, Result, Read, Write};
use std::path::{Path, PathBuf};
use rpassword::prompt_password;
use crate::{Input, Signet, System};

pub struct LocalSystem;

pub fn signet(root: PathBuf) -> Signet<LocalSystem> {
    Signet::new(root)
}

impl System for LocalSystem {
    fn init(path: &Path, data: &[u8]) -> Result<()> {
        let mut file = create(path)?;
        file.write_all(data)?;
        file.sync_all()
    }

    fn sync(path: &Path, data: &[u8]) -> Result<()> {
        let new = &path.with_extension("new");
        let old = &path.with_extension("old");

        let perms = fs::metadata(path)?.permissions();

        let mut file = create(new)?;
        file.set_permissions(perms)?;
        file.write_all(data)?;
        file.sync_all()?;

        fs::rename(path, old)?;
        fs::rename(new, path)?;
        fs::remove_file(old)?;

        Ok(())
    }

    fn read(input: &Input) -> Result<Vec<u8>> {
        match input {
            Input::File(path) => read(File::open(path)?),
            Input::Stdin      => read(stdin()),
        }
    }

    fn write(path: &Path, data: &[u8]) -> Result<()> {
        fs::write(path, data)
    }

    fn mkdir(path: &Path) -> Result<()> {
        fs::create_dir_all(path)
    }

    fn prompt(prompt: &str) -> Result<String> {
        prompt_password(prompt)
    }
}

fn create(path: &Path) -> Result<File> {
    let mut open = OpenOptions::new();
    open.create_new(true).write(true);
    open.open(path)
}

fn read<T: Read>(mut input: T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    input.read_to_end(&mut vec)?;
    Ok(vec)
}
