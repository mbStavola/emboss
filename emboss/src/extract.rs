use std::{borrow::Cow, collections::HashMap};

use emboss_common::EmbossingOptions;

use crate::error::EmbossError;

/// Extract embossed metadata from the raw bytes in a section
///
/// The metadata is expected to be written in the following format:
///
/// <IDENTIFIER>=<VALUE>\0
///
/// Where <IDENTIFIER> is the name of the identifier, <VALUE> is the embossed value,
/// and \0 is a null byte
///
/// Please note that as this format assumes identifiers ending with '=', identifiers may
/// not contain an equal sign
///
/// Example usage:
///
/// ```
/// // Example pulled directly from an example binary
/// use emboss_common::EmbossingOptions;
/// let data = "VERGEN_RUSTC_CHANNEL=stable\0VERGEN_RUSTC_COMMIT_DATE=2021-05-09\0";
///
/// let metadata = emboss::extract::extract_metadata(
///     data,
///     EmbossingOptions::default(),
/// ).unwrap();
///
/// let value = metadata.get("VERGEN_RUSTC_CHANNEL").unwrap();
/// assert_eq!(value, "stable");
///
/// let value = metadata.get("VERGEN_RUSTC_COMMIT_DATE").unwrap();
/// assert_eq!(value, "2021-05-09");
/// ```
pub fn extract_metadata(
    buf: &str,
    EmbossingOptions {
        separator,
        terminator,
        ..
    }: EmbossingOptions,
) -> Result<HashMap<Cow<str>, Cow<str>>, EmbossError> {
    let mut metadata = HashMap::new();

    let mut ident = None;
    let mut start = 0;
    let mut parsing_ident = true;

    for (i, c) in buf.chars().enumerate() {
        if c == separator && parsing_ident {
            // We've found a separator sign so the identifier is finished
            let raw_ident = Cow::from(&buf[start..i]);
            if raw_ident.trim().is_empty() {
                return Err(EmbossError::MissingIdent);
            }

            ident = Some(raw_ident);
            start = i + 1;
            parsing_ident = false;
        } else if c == terminator && parsing_ident {
            // We've reached an unexpected terminator while parsing the identifier... something is wrong
            return Err(EmbossError::UnexpectedValueEnd);
        } else if c == terminator {
            // We've hit a terminator while extracting the value-- we're done, the string is complete
            let ident = ident.take().expect("we should have an ident by now");
            let value = Cow::from(&buf[start..i]);

            metadata.insert(ident, value);
            start = i + 1;
            parsing_ident = true;
        }
    }

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_basic() {
        let data = "key=value\0";
        let metadata = extract_metadata(data, EmbossingOptions::default()).unwrap();
        let value = metadata.get("key").unwrap();
        assert_eq!(value, "value")
    }

    #[test]
    fn extract_with_eq_sign_in_value() {
        let data = "expr=2+2=4\0";
        let metadata = extract_metadata(data, EmbossingOptions::default()).unwrap();
        let value = metadata.get("expr").unwrap();
        assert_eq!(value, "2+2=4")
    }

    #[test]
    fn extract_fail_on_unfinished_ident() {
        let data = "expr\0";
        if let Err(error) = extract_metadata(data, EmbossingOptions::default()) {
            return assert_eq!(error, EmbossError::UnexpectedValueEnd);
        }

        panic!("expected an error to be returned")
    }

    #[test]
    fn extract_fail_blank_ident() {
        let data = "=foo";
        if let Err(error) = extract_metadata(data, EmbossingOptions::default()) {
            return assert_eq!(error, EmbossError::MissingIdent);
        }

        panic!("expected an error to be returned")
    }

    #[test]
    fn extract_fail_blank_ident_with_spaces() {
        let data = "  =foo";
        if let Err(error) = extract_metadata(data, EmbossingOptions::default()) {
            return assert_eq!(error, EmbossError::MissingIdent);
        }

        panic!("expected an error to be returned")
    }
}
