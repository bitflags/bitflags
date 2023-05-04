use core::fmt::{Write, self, LowerHex};

use crate::{BitFlags, Bits};

pub fn to_writer<B: BitFlags>(flags: &B, mut writer: impl Write) -> Result<(), fmt::Error>
where
    B::Bits: LowerHex,
{
    // A formatter for bitflags that produces text output like:
    //
    // A | B | 0xf6
    //
    // The names of set flags are written in a bar-separated-format,
    // followed by a hex number of any remaining bits that are set
    // but don't correspond to any flags.
    
    let mut state = flags.bits();

    // Iterate over the valid flags
    let mut first = true;
    let mut iter = flags.iter_names();
    for (name, flag) in &mut iter {
        state = state & !flag.bits();
        
        if !first {
            writer.write_str(" | ")?;
        }

        first = false;
        writer.write_str(name)?;
    }

    // Append any extra bits that correspond to flags to the end of the format
    if state != B::Bits::EMPTY {
        if !first {
            writer.write_str(" | ")?;
        }

        write!(writer, "{:#x}", state)?;
    }

    fmt::Result::Ok(())
}
