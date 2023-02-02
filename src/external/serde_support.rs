use core::{fmt, str};
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub fn serialize_bits_default<T: fmt::Display + AsRef<B>, B: Serialize, S: Serializer>(
    flags: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if serializer.is_human_readable() {
        // Serialize human-readable flags as a string like `"A | B"`
        serializer.collect_str(flags)
    } else {
        // Serialize non-human-readable flags directly as the underlying bits
        flags.as_ref().serialize(serializer)
    }
}

pub fn deserialize_bits_default<
    'de,
    T: str::FromStr + From<B>,
    B: Deserialize<'de>,
    D: Deserializer<'de>,
>(
    deserializer: D,
) -> Result<T, D::Error>
where
    <T as str::FromStr>::Err: fmt::Display,
{
    if deserializer.is_human_readable() {
        // Deserialize human-readable flags by parsing them from strings like `"A | B"`
        struct FlagsVisitor<T>(core::marker::PhantomData<T>);

        impl<'de, T: str::FromStr> Visitor<'de> for FlagsVisitor<T>
        where
            <T as str::FromStr>::Err: fmt::Display,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string value of `|` separated flags")
            }

            fn visit_str<E: Error>(self, flags: &str) -> Result<Self::Value, E> {
                flags.parse().map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_str(FlagsVisitor(Default::default()))
    } else {
        // Deserialize non-human-readable flags directly from the underlying bits
        let bits = B::deserialize(deserializer)?;

        Ok(bits.into())
    }
}

#[cfg(test)]
mod tests {
    bitflags! {
        #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
        struct SerdeFlags: u32 {
            const A = 1;
            const B = 2;
            const C = 4;
            const D = 8;
        }
    }

    #[test]
    fn test_serde_bitflags_default_serialize() {
        let flags = SerdeFlags::A | SerdeFlags::B;

        let serialized = serde_json::to_string(&flags).unwrap();

        assert_eq!(serialized, r#""A | B""#);
    }

    #[test]
    fn test_serde_bitflags_default_deserialize() {
        let deserialized: SerdeFlags = serde_json::from_str(r#""C | D""#).unwrap();

        let expected = SerdeFlags::C | SerdeFlags::D;

        assert_eq!(deserialized.bits(), expected.bits());
    }

    #[test]
    fn test_serde_bitflags_default_roundtrip() {
        let flags = SerdeFlags::A | SerdeFlags::B;

        let deserialized: SerdeFlags =
            serde_json::from_str(&serde_json::to_string(&flags).unwrap()).unwrap();

        assert_eq!(deserialized.bits(), flags.bits());
    }
}
