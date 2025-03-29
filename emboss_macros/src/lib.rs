mod codegen;
mod macro_impl;

use proc_macro::TokenStream;

/// Embosses a single key-value pair into your binary.
///
/// # Example
/// ```rust
/// # use emboss_macros::*;
/// emboss!(key = "app-version", value = "1.0.0");
/// ```
///
/// # Options
/// - `key` (required): A string literal specifying the identifier for your embossed value.
/// - `value` (required): A string literal containing the value to embed.
/// - `variant_name` (optional): Customize the enum variant name when using the generated enum.
/// - `stored_in` (optional): Specify the section where the embossed data will be stored. Default is `.emboss.meta`.
/// - `export_name` (optional): Create a module with the specified name to access embossed data via a public API.
///
/// # Extended Example
/// ```rust
/// # use emboss_macros::*;
/// emboss!(
///     key = "build-date",
///     value = "2023-05-20",
///     variant_name = "BuildDate",
///     stored_in = ".custom_section",
///     export_name = "single_build_info"
/// );
///
/// // When using export_name, you can access the value via:
/// let (key, value) = single_build_info::EMBOSSED.get_by_key("build-date").unwrap();
/// // Or using the enum variant if variant_name was provided
/// let (key, value) = single_build_info::EMBOSSED.get_by_kind(single_build_info::EmbossedKeyKind::BuildDate);
/// ```
///
/// # Notes
/// - Each section name must be unique within your codebase.
/// - When using the enum access method (`get_by_kind`), all keys must have valid variant names.
///
/// # Platform-Specific Options
/// On macOS, the `segment` parameter can be specified in addition to `stored_in` to
/// control the Mach-O segment where data is stored:
///
/// ```rust
/// # use emboss_macros::*;
/// emboss!(
///     key = "macos-release",
///     value = "14.0",
///     segment = "__DATA",
///     stored_in = "__single_emboss"
/// );
/// ```
#[proc_macro]
pub fn emboss(input: TokenStream) -> TokenStream {
    macro_impl::emboss(input)
}

/// Embosses multiple key-value pairs in a single operation.
///
/// This macro allows embedding multiple pieces of metadata in a single binary section,
/// providing efficient storage and access to related information.
///
/// # Example
/// ```rust
/// # use emboss_macros::*;
/// emboss_many!(items = [
///     { key = "release-version", value = "2.3.1" },
///     { key = "release-timestamp", value = "1620358911" }
/// ]);
/// ```
///
/// # Options
/// - `items` (required): An array of objects, each with:
///   - `key` (required): String identifier for the embossed value.
///   - `value` (required): String value to embed.
///   - `variant_name` (optional): Custom enum variant name for this item.
/// - `stored_in` (optional): Section where embossed data will be stored. Default is `.emboss.meta`.
/// - `export_name` (optional): Module name for accessing the data via a public API.
///
/// # Extended Example
/// ```rust
/// # use emboss_macros::*;
/// emboss_many!(
///     items = [
///         { key = "app-version", value = "3.2.0", variant_name = "AppVersion" },
///         { key = "git-commit", value = "a1b2c3d", variant_name = "GitCommit" }
///     ],
///     stored_in = "__many_emboss",
///     export_name = "multi_app_info"
/// );
///
/// // Access via generated module
/// let (_, version) = multi_app_info::EMBOSSED.get_by_key("app-version").unwrap();
/// // Access via enum variant
/// let (_, commit) = multi_app_info::EMBOSSED.get_by_kind(multi_app_info::EmbossedKeyKind::GitCommit);
/// // Access by index
/// let (key, value) = multi_app_info::EMBOSSED.get_by_index(1).unwrap();
/// ```
///
/// # Notes
/// - A maximum of 255 items can be embossed in a single section.
/// - Each section name must be unique within your codebase.
/// - When using the enum access method (`get_by_kind`), all keys must have valid variant names.
///
/// # Platform-Specific Options
/// On macOS, the `segment` parameter can be specified in addition to `stored_in` to
/// control the Mach-O segment where data is stored:
///
/// ```rust
/// # use emboss_macros::*;
/// emboss_many!(
///     items = [
///         { key = "build-os", value = "macOS", variant_name = "BuildOS" },
///         { key = "build-arch", value = "arm64", variant_name = "BuildArch" }
///     ],
///     segment = "__DATA",
///     stored_in = "__many_platform",
///     export_name = "platform_info"
/// );
/// ```
#[proc_macro]
pub fn emboss_many(input: TokenStream) -> TokenStream {
    macro_impl::emboss_many(input)
}

/// Embosses the value of an environment variable.
///
/// This macro captures the value of an environment variable at compile time and
/// embosses it into the binary.
///
/// # Example
/// ```rust
/// # use emboss_macros::*;
/// emboss_env!(env_var = "CARGO_PKG_VERSION", key = "rust-version");
/// ```
///
/// # Options
/// - `env_var` (required): Name of the environment variable to embed.
/// - `key` (optional): Custom key for the embossed value. Defaults to the environment variable name.
/// - `variant_name` (optional): Custom enum variant name when using the generated enum.
/// - `fallback` (optional): Behavior when the environment variable is not set:
///   - Default: Fail at compile time if the variable is not set.
///   - `fallback = "empty"`: Use an empty string if the variable is not set.
///   - `fallback = { value = "default_value" }`: Use the specified default value.
/// - `stored_in` (optional): Section where embossed data will be stored. Default is `.emboss.meta`.
/// - `export_name` (optional): Module name for accessing the data via a public API.
///
/// # Extended Example
/// ```rust
/// # use emboss_macros::*;
/// emboss_env!(
///     env_var = "CARGO_PKG_NAME",
///     key = "package-name",
///     variant_name = "PackageName",
///     fallback = Value("unknown-package"),
///     export_name = "package_info"
/// );
///
/// // When using export_name, you can access the value:
/// let (_, name) = package_info::EMBOSSED.get_by_key("package-name").unwrap();
/// // Or using the enum variant if specified:
/// let (_, name) = package_info::EMBOSSED.get_by_kind(package_info::EmbossedKeyKind::PackageName);
/// ```
///
/// # Notes
/// - Each section name must be unique within your codebase.
/// - When using the enum access method (`get_by_kind`), all keys must have valid variant names.
///
/// # Platform-Specific Options
/// On macOS, the `segment` parameter can be specified in addition to `stored_in` to
/// control the Mach-O segment where data is stored:
///
/// ```rust
/// # use emboss_macros::*;
/// emboss_env!(
///     env_var = "CARGO_CFG_TARGET_OS",
///     key = "target-os",
///     variant_name = "TargetOS",
///     fallback = Empty,
///     export_name = "build_info",
///     segment = "__DATA",
///     stored_in = "__env_emboss"
/// );
/// ```
#[proc_macro]
pub fn emboss_env(input: TokenStream) -> TokenStream {
    macro_impl::emboss_env(input)
}

/// Embosses multiple environment variables in a single operation.
///
/// This macro captures the values of multiple environment variables at compile time
/// and embosses them into the binary, making them available at runtime.
///
/// # Example
/// ```rust
/// # use emboss_macros::*;
/// emboss_envs!(env_vars = [
///     { env_var = "CARGO_PKG_NAME" },
///     { env_var = "CARGO_PKG_VERSION" }
/// ]);
/// ```
///
/// # Options
/// - `env_vars` (required): An array of objects, each with:
///   - `env_var` (required): Environment variable name to embed.
///   - `key` (optional): Custom key for this variable. Defaults to the environment variable name.
///   - `variant_name` (optional): Custom enum variant name.
///   - `fallback` (optional): Behavior when the variable is not set (same options as `emboss_env!`).
/// - `stored_in` (optional): Section where embossed data will be stored. Default is `.emboss.meta`.
/// - `export_name` (optional): Module name for accessing the data via a public API.
///
/// # Extended Example
/// ```rust
/// # use emboss_macros::*;
/// emboss_envs!(
///     env_vars = [
///         {
///             env_var = "CARGO_PKG_VERSION",
///             key = "version",
///             fallback = Value("https://github.com/example/example"),
///             variant_name = "Version"
///         },
///         {
///             env_var = "CARGO_PKG_REPOSITORY",
///             fallback = Value("0.5.0"),
///             variant_name = "Repository"
///         }
///     ],
///     stored_in = "__envs_emboss",
///     export_name = "cargo_metadata"
/// );
///
/// // Access values:
/// let (_, version) = cargo_metadata::EMBOSSED.get_by_key("version").unwrap();
/// let (_, repo) = cargo_metadata::EMBOSSED.get_by_kind(cargo_metadata::EmbossedKeyKind::Repository);
/// ```
///
/// # Notes
/// - A maximum of 255 items can be embossed in a single section.
/// - Each section name must be unique within your codebase.
/// - Keys used for embossing should be valid Rust identifiers if you intend to use them with enum variants.
///
/// # Platform-Specific Options
/// On macOS, the `segment` parameter can be specified in addition to `stored_in` to
/// control the Mach-O segment where data is stored:
///
/// ```rust
/// # use emboss_macros::*;
/// emboss_envs!(
///     env_vars = [
///         {
///             env_var = "CARGO_PKG_NAME",
///             key = "name",
///             variant_name = "Name"
///         },
///         {
///             env_var = "CARGO_CFG_TARGET_ARCH",
///             fallback = Empty,
///             variant_name = "TargetArch"
///         }
///     ],
///     segment = "__DATA",
///     stored_in = "__many_env",
///     export_name = "build_metadata"
/// );
/// ```
#[proc_macro]
pub fn emboss_envs(input: TokenStream) -> TokenStream {
    macro_impl::emboss_envs(input)
}
