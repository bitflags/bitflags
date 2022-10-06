use core::fmt;
use serde::{
    de::{Error, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

// These methods are compatible with the result of `#[derive(Serialize, Deserialize)]` on bitflags `1.0` types

pub fn serialize_bits_default<B: Serialize, S: Serializer>(
    name: &'static str,
    bits: &B,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut serialize_struct = serializer.serialize_struct(name, 1)?;
    serialize_struct.serialize_field("bits", bits)?;
    serialize_struct.end()
}

pub fn deserialize_bits_default<'de, B: Deserialize<'de>, D: Deserializer<'de>>(
    name: &'static str,
    deserializer: D,
) -> Result<B, D::Error> {
    struct BitsVisitor<T>(core::marker::PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for BitsVisitor<T> {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a primitive bitflags value wrapped in a struct")
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut bits = None;

            while let Some(key) = map.next_key()? {
                match key {
                    "bits" => {
                        if bits.is_some() {
                            return Err(Error::duplicate_field("bits"));
                        }

                        bits = Some(map.next_value()?);
                    }
                    v => return Err(Error::unknown_field(v, &["bits"])),
                }
            }

            bits.ok_or_else(|| Error::missing_field("bits"))
        }
    }

    deserializer.deserialize_struct(name, &["bits"], BitsVisitor(Default::default()))
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

        assert_eq!(serialized, r#"{"bits":3}"#);
    }

    #[test]
    fn test_serde_bitflags_default_deserialize() {
        let deserialized: SerdeFlags = serde_json::from_str(r#"{"bits":12}"#).unwrap();

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
