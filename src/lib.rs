use std::{borrow::Cow, collections::HashMap};

pub use error::*;

mod error;

pub const LINUX_SECTION_NAME: &str = include_str!("linux_section.txt");
pub const MACOS_SECTION_NAME: &str = include_str!("macos_section.txt");

#[cfg(target_os = "linux")]
pub const DEFAULT_SECTION_NAME: &str = LINUX_SECTION_NAME;
#[cfg(target_os = "macos")]
pub const DEFAULT_SECTION_NAME: &str = MACOS_SECTION_NAME;

const IDENT_END: u8 = b'=';
const VALUE_END: u8 = b'\0';

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
/// let data = "VERGEN_RUSTC_CHANNEL=stable\0VERGEN_RUSTC_COMMIT_DATE=2021-05-09\0";
///
/// let metadata = emboss::extract_metadata(data.as_bytes()).unwrap();
///
/// let value = metadata.get("VERGEN_RUSTC_CHANNEL").unwrap();
/// assert_eq!(value, "stable");
///
/// let value = metadata.get("VERGEN_RUSTC_COMMIT_DATE").unwrap();
/// assert_eq!(value, "2021-05-09");
/// ```
pub fn extract_metadata(buf: &[u8]) -> Result<HashMap<Cow<str>, Cow<str>>, EmbossError> {
    let mut metadata = HashMap::new();

    let mut ident = None;
    let mut start = 0;
    let mut parsing_ident = true;

    for (i, c) in buf.iter().enumerate() {
        match *c {
            // We've found an equal sign so the identifier is finished
            IDENT_END if parsing_ident => {
                let raw_ident = String::from_utf8_lossy(&buf[start..i]);
                if raw_ident.trim().is_empty() {
                    return Err(EmbossError::MissingIdent);
                }

                ident = Some(raw_ident);
                start = i + 1;
                parsing_ident = false;
            }
            // We've reached an unexpected null byte while parsing the identifier... somethings wrong
            VALUE_END if parsing_ident => {
                return Err(EmbossError::UnexpectedValueEnd);
            }
            // We've hit a null byte while extracting the value-- we're done, the string is complete
            VALUE_END => {
                let ident = ident.take().expect("we should have an ident by now");
                let value = String::from_utf8_lossy(&buf[start..i]);

                metadata.insert(ident, value);
                start = i + 1;
                parsing_ident = true;
            }
            _ => {}
        }
    }

    Ok(metadata)
}

#[macro_export]
macro_rules! emboss {
    (groups=$($group: ident),+) => {
        $(
            emboss!(group=$group);
        )+
    };
    (group=build) => {
        emboss!(VERGEN_BUILD_DATE);
        emboss!(VERGEN_BUILD_TIME);
        emboss!(VERGEN_BUILD_TIMESTAMP);
        emboss!(VERGEN_BUILD_SEMVER);
    };
    (group=git) => {
        emboss!(VERGEN_GIT_BRANCH);
        emboss!(VERGEN_GIT_COMMIT_DATE);
        emboss!(VERGEN_GIT_COMMIT_TIME);
        emboss!(VERGEN_GIT_COMMIT_TIMESTAMP);
        emboss!(VERGEN_GIT_SEMVER);
        emboss!(VERGEN_GIT_SEMVER_LIGHTWEIGHT);
        emboss!(VERGEN_GIT_SHA);
        emboss!(VERGEN_GIT_SHA_SHORT);
    };
    (group=rustc) => {
        emboss!(VERGEN_RUSTC_CHANNEL);
        emboss!(VERGEN_RUSTC_COMMIT_DATE);
        emboss!(VERGEN_RUSTC_COMMIT_HASH);
        emboss!(VERGEN_RUSTC_HOST_TRIPLE);
        emboss!(VERGEN_RUSTC_LLVM_VERSION);
        emboss!(VERGEN_RUSTC_SEMVER);
    };
    (group=cargo) => {
        emboss!(VERGEN_CARGO_FEATURES);
        emboss!(VERGEN_CARGO_PROFILE);
        emboss!(VERGEN_CARGO_TARGET_TRIPLE);
    };
    (group=rust) => {
        emboss!(groups=rustc,cargo);
    };
    (group=sysinfo) => {
        emboss!(VERGEN_SYSINFO_NAME);
        emboss!(VERGEN_SYSINFO_OS_VERSION);
        emboss!(VERGEN_SYSINFO_USER);
        emboss!(VERGEN_SYSINFO_TOTAL_MEMORY);
        emboss!(VERGEN_SYSINFO_CPU_VENDOR);
        emboss!(VERGEN_SYSINFO_CPU_CORE_COUNT);
        emboss!(VERGEN_SYSINFO_CPU_NAME);
        emboss!(VERGEN_SYSINFO_CPU_BRAND);
        emboss!(VERGEN_SYSINFO_CPU_FREQUENCY);
    };
    (group=all) => {
        emboss!(groups=build,git,rustc,cargo,sysinfo);
    };
    (group=rsps) => {
        emboss!(VERGEN_BUILD_TIMESTAMP);
        emboss!(VERGEN_BUILD_SEMVER);

        emboss!(VERGEN_RUSTC_SEMVER);
        emboss!(VERGEN_CARGO_PROFILE);
        emboss!(VERGEN_CARGO_FEATURES);
    };
    ($var_name: ident) => {
        #[cfg(target_os = "linux")]
        emboss!($var_name, include_str!("linux_section.txt"));

        #[cfg(target_os = "macos")]
        emboss!($var_name, include_str!("macos_section.txt"));
    };
    ($var_name: ident, $section_name: expr) => {
        emboss!($var_name, $section_name, env!(stringify!($var_name)));
    };
    // Some interesting things going on in this macro! See:
    //  Tricky bits with expanding in attrs: https://github.com/rust-lang/rust/pull/83366
    //  Using modules instead of vars: https://github.com/rust-lang/rust/issues/29599
    //  On Transmuting: https://github.com/rust-lang/rust/issues/70239
    //  Disabling the transmute lint: https://rust-lang.github.io/rust-clippy/master/index.html#transmute_ptr_to_ref
    ($var_name: ident, $section_name: expr, $value: expr) => {
        mod $var_name {
            type Data = [u8; STRUCTURED.as_bytes().len()];

            const STRUCTURED: &str = concat!(stringify!($var_name), "=", $value, "\0");

            #[used]
            #[link_section = $section_name]
            static EMBOSSED: Data = unsafe {
                #[allow(clippy::transmute_ptr_to_ref)]
                *std::mem::transmute::<*const Data, &Data>(STRUCTURED.as_ptr() as *const Data)
            };
        }
    };
    () => {
        emboss!(group=rsps);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_basic() {
        let data = "key=value\0";
        let metadata = extract_metadata(data.as_bytes()).unwrap();
        let value = metadata.get("key").unwrap();
        assert_eq!(value, "value")
    }

    #[test]
    fn extract_with_eq_sign_in_value() {
        let data = "expr=2+2=4\0";
        let metadata = extract_metadata(data.as_bytes()).unwrap();
        let value = metadata.get("expr").unwrap();
        assert_eq!(value, "2+2=4")
    }

    #[test]
    fn extract_fail_on_unfinished_ident() {
        let data = "expr\0";
        if let Err(error) = extract_metadata(data.as_bytes()) {
            return assert_eq!(error, EmbossError::UnexpectedValueEnd);
        }

        panic!("expected an error to be returned")
    }

    #[test]
    fn extract_fail_blank_ident() {
        let data = "=foo";
        if let Err(error) = extract_metadata(data.as_bytes()) {
            return assert_eq!(error, EmbossError::MissingIdent);
        }

        panic!("expected an error to be returned")
    }

    #[test]
    fn extract_fail_blank_ident_with_spaces() {
        let data = "  =foo";
        if let Err(error) = extract_metadata(data.as_bytes()) {
            return assert_eq!(error, EmbossError::MissingIdent);
        }

        panic!("expected an error to be returned")
    }
}
