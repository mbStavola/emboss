use std::{error::Error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum EmbossError {
    UnexpectedValueEnd,
    MissingIdent,
}

impl fmt::Display for EmbossError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmbossError::UnexpectedValueEnd => {
                write!(
                    f,
                    "Prematurely reached the end of metadata value during extraction"
                )
            }
            EmbossError::MissingIdent => {
                write!(
                    f,
                    "Metadata identifier is either blank or comprised entirely of whitespace"
                )
            }
        }
    }
}

impl Error for EmbossError {}
