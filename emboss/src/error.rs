use std::{error::Error, fmt, str::Utf8Error};

#[derive(Debug, Eq, PartialEq)]
pub enum EmbossError {
    EmptyEmbossing,
    IncorrectLeadingMagic,
    InvalidEmbossedCString,
    InvalidUtf8(Utf8Error),
}

impl fmt::Display for EmbossError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmbossError::EmptyEmbossing => {
                write!(f, "Empty Embossing")
            }
            EmbossError::IncorrectLeadingMagic => {
                write!(
                    f,
                    "Leading bytes did not correspond to a valid Emboss magic number"
                )
            }
            EmbossError::InvalidEmbossedCString => {
                write!(
                    f,
                    "No terminating null-byte was found in the embossed string"
                )
            }
            EmbossError::InvalidUtf8(err) => err.fmt(f),
        }
    }
}

impl Error for EmbossError {}
