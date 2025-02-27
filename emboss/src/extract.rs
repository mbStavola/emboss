use std::{collections::HashMap, ffi::CStr};

use emboss_common::LEADING_MAGIC_BYTES;

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
/// let data =
///     b"\x55\xB0\x77\x1A\x01VERGEN_RUSTC_CHANNEL\0stable\0VERGEN_RUSTC_COMMIT_DATE\02021-05-09\0";
///
/// let metadata = emboss::extract::extract_metadata_into_hashmap(data).unwrap();
///
/// let value = *metadata.get("VERGEN_RUSTC_CHANNEL").unwrap();
/// assert_eq!(value, "stable");
///
/// let value = *metadata.get("VERGEN_RUSTC_COMMIT_DATE").unwrap();
/// assert_eq!(value, "2021-05-09");
/// ```
pub fn extract_metadata_into_vec(buf: &[u8]) -> Result<Vec<(&str, &str)>, EmbossError> {
    extract_metadata(buf)
}

pub fn extract_metadata_into_hashmap(buf: &[u8]) -> Result<HashMap<&str, &str>, EmbossError> {
    let items = extract_metadata(buf)?
        .iter()
        .fold(HashMap::new(), |mut map, (key, value)| {
            map.insert(*key, *value);
            map
        });
    Ok(items)
}

// TODO(Matt): This is internal with a transparent passthrough via extract_metadata_into_vec
//  simply because we want generators to stabilize so that we can just yield pairs instead
//  of allocating the Vec
//  Once we can do that on stable Rust, we'll refactor this function and then expose it in
//  the public API
fn extract_metadata(buf: &[u8]) -> Result<Vec<(&str, &str)>, EmbossError> {
    let leading_magic_matches = buf
        .first_chunk::<4>()
        .map(|chunk| u32::from_le_bytes(*chunk))
        .filter(|bytes| *bytes == LEADING_MAGIC_BYTES)
        .is_some();

    if !leading_magic_matches {
        return Err(EmbossError::IncorrectLeadingMagic);
    }

    let item_count = buf[4] as usize;
    if item_count == 0 {
        return Err(EmbossError::EmptyEmbossing);
    }

    let mut content_buf = &buf[5..];
    let mut items = Vec::with_capacity(item_count);

    let mut current_key = None;
    while !content_buf.is_empty() {
        let str_slice = CStr::from_bytes_until_nul(content_buf)
            .map_err(|_| EmbossError::InvalidEmbossedCString)?;

        let str_slice = str_slice.to_str().map_err(EmbossError::InvalidUtf8)?;

        if let Some(key) = current_key.take() {
            items.push((key, str_slice));
        } else {
            current_key = Some(str_slice);
        }

        content_buf = &content_buf[str_slice.len() + 1..];
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_basic() {
        let data = b"\x55\xB0\x77\x1A\x01key\0value\0";
        let metadata = extract_metadata_into_hashmap(data).unwrap();
        let value = metadata.get("key").unwrap();
        assert_eq!(*value, "value")
    }

    #[test]
    fn extract_fail_incorrect_magic() {
        let data = b"\x44\xB0\x77\x1A\x01key\0value\0";
        if let Err(error) = extract_metadata_into_hashmap(data) {
            return assert_eq!(error, EmbossError::IncorrectLeadingMagic);
        }

        panic!("expected an error to be returned")
    }

    #[test]
    fn extract_fail_on_empty_data() {
        let data = b"\x55\xB0\x77\x1A\x00";
        if let Err(error) = extract_metadata_into_hashmap(data) {
            return assert_eq!(error, EmbossError::EmptyEmbossing);
        }

        panic!("expected an error to be returned")
    }

    #[test]
    fn extract_fail_invalid_utf8() {
        let data = &[0x55, 0xB0, 0x77, 0x1A, 0x01, 0xB0, 0x55, 0x0, 0x49, 0x0];
        if let Err(error) = extract_metadata_into_hashmap(data) {
            return assert!(matches!(error, EmbossError::InvalidUtf8(_)));
        }

        panic!("expected an error to be returned")
    }

    #[test]
    fn extract_fail_invalid_c_string() {
        let data = b"\x55\xB0\x77\x1A\x01foo\0bar";
        if let Err(error) = extract_metadata_into_hashmap(data) {
            return assert_eq!(error, EmbossError::InvalidEmbossedCString);
        }

        panic!("expected an error to be returned")
    }
}
