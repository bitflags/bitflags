//! Formatting flags as text.
//!
//! For details on the format, see the [`parser`](../parser/index.html) module.

use core::fmt::{Write, self, LowerHex};

use crate::{Flags, Bits};

/// Write a set of flags to a writer.
///
/// Any bits that don't correspond to a valid flag will be formatted
/// as a hex number.
pub fn to_writer<B: Flags>(flags: &B, mut writer: impl Write) -> Result<(), fmt::Error>
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

    // Iterate over the valid flags
    let mut first = true;
    let mut iter = flags.iter_names();
    for (name, _) in &mut iter {
        if !first {
            writer.write_str(" | ")?;
        }

        first = false;
        writer.write_str(name)?;
    }

    // Append any extra bits that correspond to flags to the end of the format
    let remaining = iter.remaining().bits();
    if remaining != B::Bits::EMPTY {
        if !first {
            writer.write_str(" | ")?;
        }

        write!(writer, "{:#x}", remaining)?;
    }

    fmt::Result::Ok(())
}
