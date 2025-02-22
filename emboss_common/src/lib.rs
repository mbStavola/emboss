use serde::Deserialize;

#[cfg(target_os = "macos")]
pub const DEFAULT_SEGMENT_NAME: &str = "__DATA";

pub const DEFAULT_SECTION_NAME: &str = ".emboss.meta";

pub const DEFAULT_SEPARATOR: char = '=';
pub const DEFAULT_TERMINATOR: char = '\0';

#[derive(Deserialize, Clone, Debug)]
pub struct EmbossingOptions {
    #[serde(default = "get_default_section_name")]
    pub stored_in: String,

    #[serde(default = "get_default_separator")]
    pub separator: char,

    #[serde(default = "get_default_terminator")]
    pub terminator: char,

    #[cfg(target_os = "macos")]
    #[serde(default = "get_default_segment_name")]
    pub segment: String,
}

impl Default for EmbossingOptions {
    fn default() -> Self {
        Self {
            stored_in: DEFAULT_SECTION_NAME.to_string(),
            separator: DEFAULT_SEPARATOR,
            terminator: DEFAULT_TERMINATOR,
            #[cfg(target_os = "macos")]
            segment: DEFAULT_SEPARATOR.to_string(),
        }
    }
}

#[cfg(target_os = "macos")]
fn get_default_segment_name() -> String {
    DEFAULT_SEGMENT_NAME.to_string()
}

fn get_default_section_name() -> String {
    DEFAULT_SECTION_NAME.to_string()
}

fn get_default_separator() -> char {
    DEFAULT_SEPARATOR
}

fn get_default_terminator() -> char {
    DEFAULT_TERMINATOR
}
