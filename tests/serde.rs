#![cfg(feature = "serde")]

use bitflags::bitflags;
use serde_test::{Token, assert_tokens};

bitflags! {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct SerdeFlags: u32 {
        const A = 1;
        const B = 2;
        const C = 4;
        const D = 8;
    }
}

#[test]
fn test_serde_bitflags_serialize() {
    let flags = SerdeFlags::A | SerdeFlags::B;

    let serialized = serde_json::to_string(&flags).unwrap();

    assert_eq!(serialized, r#"{"bits":3}"#);
}

#[test]
fn test_serde_bitflags_deserialize() {
    let deserialized: SerdeFlags = serde_json::from_str(r#"{"bits":12}"#).unwrap();

    let expected = SerdeFlags::C | SerdeFlags::D;

    assert_eq!(deserialized.bits(), expected.bits());
}
