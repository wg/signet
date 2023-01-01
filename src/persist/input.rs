use std::path::Path;
use crate::{Input, System};
use super::{Context, Error};

pub fn read<S: System>(input: &Input) -> Result<Vec<u8>, Error> {
    match input {
        Input::File(path) => S::read(input).context(path),
        Input::Stdin      => S::read(input).context(Path::new("")),
    }
}
