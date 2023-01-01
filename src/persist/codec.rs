use std::fmt;
use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::de::{EnumAccess, Expected, Unexpected, VariantAccess};
use serde::ser::{Serialize, Serializer};
use ssh_key::{LineEnding, PrivateKey};
use crate::keychain::Key;

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_enum("", &["SSH"], KeyVisitor)
    }
}

impl Serialize for Key {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;

        let Key::SSH(key) = self;

        let encoded = match key.to_openssh(LineEnding::default()) {
            Ok(key) => key,
            Err(e)  => return Err(S::Error::custom(e)),
        };

        s.serialize_newtype_variant::<String>("", 0, "SSH", &encoded)
    }
}

struct KeyVisitor;

impl<'de> Visitor<'de> for KeyVisitor {
    type Value = Key;

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (kind, var) = data.variant::<String>()?;

        let encoded = match kind.as_str() {
            "SSH" => var.newtype_variant::<String>()?,
            kind  => return Err(A::Error::unknown_variant(kind, &["SSH"])),
        };

        match PrivateKey::from_openssh(&encoded) {
            Ok(key) => Ok(Key::SSH(key)),
            Err(_)  => Err(invalid::<A>(&encoded, &self)),
        }
    }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an OpenSSH private key")
    }
}

fn invalid<'de, A: EnumAccess<'de>>(str: &str, expected: &dyn Expected) -> A::Error {
    A::Error::invalid_value(Unexpected::Str(str), expected)
}
