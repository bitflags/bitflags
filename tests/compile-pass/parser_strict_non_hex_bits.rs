// NOTE: Missing `B::Bits: WriteHex/ParseHex`

pub fn format<B: bitflags::Flags>(flags: B) {
    let _ = bitflags::parser::to_writer_strict(&flags, String::new());
}

pub fn parse<B: bitflags::Flags>(input: &str) {
    let _ = bitflags::parser::from_str_strict::<B>(input);
}

fn main() {}
