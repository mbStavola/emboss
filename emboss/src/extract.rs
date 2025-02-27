use std::{collections::HashMap, ffi::CStr};

use emboss_common::LEADING_MAGIC_BYTES;

use crate::error::EmbossError;

/// Extract embossed metadata from the raw bytes in a section
///
/// The complete binary format is as follows:
///
/// ```
/// [MAGIC_BYTES (4 bytes)][COUNT (1 byte)][KEY_1]\0[VALUE_1]\0[KEY_2]\0[VALUE_2]\0...
/// ```
///
/// Where:
/// - `MAGIC_BYTES` are the 4 bytes defined in `emboss_common::LEADING_MAGIC_BYTES`
/// - `COUNT` is a single byte indicating how many key-value pairs are present
/// - Each key and value is a null-terminated UTF-8 string
///
/// # Example
///
/// ```rust
/// // Example of embossed data bytes
/// let data = b"\x55\xB0\x77\x1A\x02app-version\0v1.0.0\0build-id\0abc123\0";
///
/// // Parse into a vector of key-value pairs
/// let metadata = emboss::extract::extract_metadata_into_vec(data).unwrap();
///
/// // Pairs are returned in the order they were embossed
/// let (key, value) = metadata[0];
/// assert_eq!(key, "app-version");
/// assert_eq!(value, "v1.0.0");
///
/// let (key, value) = metadata[1];
/// assert_eq!(key, "build-id");
/// assert_eq!(value, "abc123");
/// ```
pub fn extract_metadata_into_vec(buf: &[u8]) -> Result<Vec<(&str, &str)>, EmbossError> {
    extract_metadata(buf)
}

/// Extract embossed metadata into a HashMap for key-based lookups
///
/// This function extracts the same data as `extract_metadata_into_vec`, but returns
/// a HashMap for efficient key-based lookups. See `extract_metadata_into_vec` for 
/// details on the expected data format.
///
/// # Example
///
/// ```rust
/// // Example of embossed data bytes  
/// let data = b"\x55\xB0\x77\x1A\x02version\01.0.0\0timestamp\01620000000\0";
///
/// // Parse into a HashMap
/// let metadata = emboss::extract::extract_metadata_into_hashmap(data).unwrap();
///
/// // Look up values by key
/// let version = metadata.get("version").unwrap();
/// assert_eq!(*version, "1.0.0");
///
/// let timestamp = metadata.get("timestamp").unwrap();
/// assert_eq!(*timestamp, "1620000000");
/// ```
///
/// # Errors
///
/// This function will return an `EmbossError` if:
///
/// - The data doesn't start with the correct magic bytes (`IncorrectLeadingMagic`)
/// - The data contains no key-value pairs (`EmptyEmbossing`)
/// - Any string in the data is missing a null terminator (`InvalidEmbossedCString`)
/// - Any string in the data isn't valid UTF-8 (`InvalidUtf8`)
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
