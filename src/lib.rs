#![allow(clippy::module_inception, clippy::redundant_field_names)]

pub use crate::keychain::Key;
pub use crate::keychain::Keychain;
pub use crate::keychain::Metadata;
pub use crate::keychain::Password;
pub use crate::keychain::Provider;

pub use crate::keystore::Keystore;
pub use crate::platform::signet;

pub use crate::secret::Secret;
pub use crate::signet::Signet;
pub use crate::system::Input;
pub use crate::system::System;

pub mod args;
pub mod command;

mod keychain;
mod keystore;
mod persist;
mod platform;
mod secret;
mod signet;
mod system;
