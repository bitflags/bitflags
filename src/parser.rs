use core::fmt;

#[derive(Debug)]
pub struct ParseError {
    #[cfg(feature = "std")]
    invalid: String,
}

impl ParseError {
    pub fn invalid_flag(flag: impl fmt::Display) -> Self {
        #[cfg(feature = "std")]
        {
            ParseError {
                invalid: flag.to_string(),
            }
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = flag;

            ParseError {}
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse a set of bitflags")?;

        #[cfg(feature = "std")]
        {
            write!(f, ": unrecognized flag `{}`", self.invalid)?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
