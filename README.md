# Emboss

![Crates.io](https://img.shields.io/crates/v/emboss)
![Crates.io](https://img.shields.io/crates/d/emboss)

![][i-emboss]

Macros to embed metadata as an ELF/Mach-O section in your final binary.

Pairs quite nicely with [vergen][vergen].

## Quickstart

Include `emboss` in your `Cargo.toml`:

```toml
[package]
# <snip>

[dependencies]
emboss = "0.5.1"
```

Import the emboss macro and call it with the key and value you want to embed:

```rust
use emboss::emboss;

emboss!(key = "my-custom-value", value = "1");
```

Run a quick `cargo build` and then examine the resulting binary:

```bash
$ strings target/debug/kana | grep my-custom-value
my-custom-value=1
```

You can also parse this yourself from the binary or use [rsps][rsps] to fetch it from a running process.

## Detailed Usage

### Emboss Many

You can emboss multiple key value pairs at once using `emboss_many`:

```rust
use emboss::emboss_many;

// The `items` property takes an array of key-value structs
emboss_many!(items = [
    { key = "foo", value = "1" },
    { key = "bar", value = "2" }
]);
```

### Emboss From Environment Variables

If you need to pull the value of a environment variable for the embossing, there is `emboss_env`:

```rust
use emboss::emboss_env;

// `env_var` will be the environment variable to evaluate at compile time
// `key` will be the key in the embossing, same as it is in the normal emboss macro
// If you omit `key`, it'll reuse the env var as the `key`
emboss_env!(env_var = "FOO_VAR", key = "foo");
```

If the environment variable is not present at compile time, the macro will fail. You can change this behavior with the `fallback` property:

```rust
use emboss::emboss_env;

// The `Fail` variant is the default behavior, blowing up when the `env_var` is missing  
emboss_env!(env_var = "DOES_NOT_EXIST", fallback = Fail);

// The `Empty` variant will use an empty value when the `env_var` is missing  
emboss_env!(env_var = "DOES_NOT_EXIST", fallback = Empty);

// The `Value` variant will use a specific, user-specified value when the `env_var` is missing
emboss_env!(env_var = "DOES_NOT_EXIST", fallback = Value("1"));
```

### Emboss Many From Environment Variables

Similar to `emboss_many`, but for values pulled from environment variables. You can use all the same properties present in `emboss_env`:

```rust
use emboss::emboss_envs;

// The `env_vars` property takes an array of environment variable spec structs
emboss_envs!(env_vars = [
    { env_var = "FOO_VAR" },
    { env_var = "BAR_VAR", key = "bar", fallback = Empty },
    { env_var = "BAZ_VAR", key = "baz", fallback = Value("3") },
]);
```

### Exports and Enum Variants

All emboss macros support the `export_name` parameter, which creates a module with a public API for accessing embossed data:

```rust
use emboss::emboss;

emboss!(
    key = "app-version", 
    value = "1.0.0", 
    export_name = "version_info"
);

// Later in your code:
let (_, version) = version_info::EMBOSSED.get_by_key("app-version").unwrap();
println!("App version: {}", version);
```

You can also customize enum variant names for better ergonomics:

```rust
use emboss::emboss_many;

emboss_many!(
    items = [
        { key = "version", value = "1.0.0", variant_name = "Version" },
        { key = "build-id", value = "abc123", variant_name = "BuildId" }
    ],
    export_name = "app_info"
);

// Access via enum variants
let (_, version) = app_info::EMBOSSED.get_by_kind(app_info::EmbossedKeyKind::Version);
let (_, build_id) = app_info::EMBOSSED.get_by_kind(app_info::EmbossedKeyKind::BuildId);
```

### Extended Arguments

All emboss macros support the following properties:

- `stored_in`: The name of the section to store embossed data. Defaults to `.emboss.meta`.

On macOS, an additional `segment` parameter allows you to customize the segment that the section is placed in:

```rust
emboss!(
    key = "macos-version", 
    value = "14.0", 
    segment = "__DATA", 
    stored_in = "__custom_section"
);
```

By default, the segment will be `__DATA`.

## Reading Embossed Data

We provide helper functions to retrieve embossed data from a given sequence of bytes.

Here is an example using the [object][object] crate:

```rust
use emboss::extract;

fn read_binary_metadata(file: &object::File) {
  // Pull the raw data from the metadata section of the binary
  let section = file.section_by_name(emboss_common::DEFAULT_SECTION_NAME)
      .expect("metadata should exist");
  let data = section.data().expect("data should be available");
  
  // Parse the embossed data
  let metadata = extract::extract_metadata_into_hashmap(data)
      .expect("should be able to parse metadata");

  // Access values by key
  let version = metadata.get("version").expect("version should be present");
  println!("Version: {}", version);
  
  // Alternatively, get as a vector of pairs
  let pairs = extract::extract_metadata_into_vec(data)
      .expect("should be able to parse metadata");
  
  for (key, value) in pairs {
      println!("{}: {}", key, value);
  }
}
```

# License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE][apache-license] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT][mit-license] or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[i-emboss]: https://github.com/mbStavola/emboss/blob/master/EMBOSS.jpg
[vergen]: https://github.com/rustyhorde/vergen
[rsps]: https://github.com/mbStavola/rsps
[object]: https://github.com/gimli-rs/object
[apache-license]: ./LICENSE-APACHE
[mit-license]: ./LICENSE-MIT