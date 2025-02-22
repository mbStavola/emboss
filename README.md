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
emboss = "0.4.0"
```

Import the emboss macro and call it with the key and value you want to embed:

```rust
use emboss::emboss;

emboss!(key = "my-custom-value", value = "1");
```

Run a quick `cargo build` and then examine the resulting binary:

```bash
$ strings target/debug/kana | grep CARGO_PKG_VERSION
my-custom-value=1
```

You can also parse this yourself from the binary or use [rsps][rsps] to fetch it from a running process.

## Detailed Usage

### Emboss Many

You can emboss multiple key value pairs at once using `emboss_many`:

```rust
use emboss::emboss_many;

// The `pairs` property takes an array of key-value tuples
emboss_many!(pairs = [("foo", "1"), ("bar", "2")]);
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

### Extended Arguments

All emboss macros support the following properties:

- `stored_in`: The name of the section to store embossed data. Defaults to `.emboss.meta`.
- `separator`: The character to use as a separator between embossed keys and their associated values. Defaults to `=`.
- `terminator`: The character to use as a terminator for embossed key-value pairs. Defaults to a null byte.

On macOS, an additional `segment` property is exposed which allows you to customize the segment that the section is placed in. By default embossed data will appear in the __DATA segment.

## Reading Embossed Data

We provide a simple helper function to retrieve embossed data from a given sequence of bytes.

Here is an example using the [object][object] crate:

```rust
fn read_binary_metadata(file: &object::File) {
  // Pull the raw data from the metadata section of the binary
  // For this example, we'll assume that the following is returned:
  //    VERGEN_RUSTC_CHANNEL=stable\0VERGEN_RUSTC_COMMIT_DATE=2021-05-09\0
  let section = file.section_by_name(emboss::DEFAULT_SECTION_NAME).expect("metadata should exist");
  let data = section.data().expect("data should be available");
  let text = String::from_utf8_lossy(data).to_string();

  let metadata = emboss::extract_metadata(&text, emboss::EmbossingOptions::default())
          .expect("should be able to parse metadata");

  let value = metadata.get("VERGEN_RUSTC_CHANNEL").expect("VERGEN_RUSTC_CHANNEL should be present");
  assert_eq!(value, "stable");

  let value = metadata.get("VERGEN_RUSTC_COMMIT_DATE").expect("VERGEN_RUSTC_COMMIT_DATE should be present");
  assert_eq!(value, "2021-05-09");
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
[env-macro-limitation]: https://github.com/rust-lang/rust/issues/48952
[object]: https://github.com/gimli-rs/object
[apache-license]: ./LICENSE-APACHE
[mit-license]: ./LICENSE-MIT