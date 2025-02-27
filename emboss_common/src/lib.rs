use serde::Deserialize;

pub const LEADING_MAGIC_BYTES: u32 = 0x1A77B055;

#[cfg(target_os = "macos")]
pub const DEFAULT_SEGMENT_NAME: &str = "__DATA";

pub const DEFAULT_SECTION_NAME: &str = ".emboss.meta";

#[derive(Deserialize, Clone, Debug)]
pub struct EmbossingOptions {
    #[serde(default = "get_default_section_name")]
    pub stored_in: String,

    pub export_name: Option<String>,

    #[cfg(target_os = "macos")]
    #[serde(default = "get_default_segment_name")]
    pub segment: String,
}

impl Default for EmbossingOptions {
    fn default() -> Self {
        Self {
            stored_in: DEFAULT_SECTION_NAME.to_string(),
            export_name: None,
            #[cfg(target_os = "macos")]
            segment: DEFAULT_SEGMENT_NAME.to_string(),
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
