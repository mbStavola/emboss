# Emboss

![Crates.io](https://img.shields.io/crates/v/emboss)
![Crates.io](https://img.shields.io/crates/d/emboss)

![][i-emboss]

A small macro to embed information from the environment into your final binary. 

The embedded information is structured in a way that also makes it easy to find and extract.

Pairs quite nicely with [vergen][vergen].

## Quickstart

Include `emboss` in your `Cargo.toml`:

```toml
[package]
# <snip>

[dependencies]
emboss = "0.1.0"
```

Import the macro and call it with the name of the environment variable you want to embed:

```rust
use emboss::emboss;

emboss!(CARGO_PKG_VERSION);
```

Run a quick `cargo build` and then examine the resulting binary:

```commandline
[LOCAL] matt@t470s ~/workspace/rust/i-emboss ‹main*› 
[17:39] $ strings target/debug/kana | grep CARGO_PKG_VERSION
CARGO_PKG_VERSION=0.1.0;
```

You can either parse this yourself from the binary or use [rsps][rsps] to fetch it from a running process.

**Note**: if the environment variable isn't present, the macro invocation will fail. If this annoys you, please see [this issue][env-macro-limitation].

## Usage with vergen

This crate has many convenience calls for use with `vergen`.

To get started, include both `vergen` and `emboss` in your `Cargo.toml`:

```toml
[package]
# <snip>
build = "build.rs"

[build-dependencies]
vergen = "5.1.5"

[dependencies]
emboss = "0.1.0"
```

Set up your `build.rs` to utilize `vergen`:

```rust
use vergen::{Config, vergen};

fn main() -> Result<(), ()> {
    let mut config = Config::default();

    vergen(config).unwrap();

    Ok(())
}
```

Finally, import and call `emboss`:

```rust
use emboss::emboss;

// Includes every rustc related env var provided by vergen
emboss!(group=rustc);
```

If all went well, following a build, you should see some `vergen` attributes in the final binary:

```commandline
[LOCAL] matt@t470s ~/workspace/rust/i-emboss ‹main*› 
[17:54] $ strings target/debug/kana | grep VERGEN           
VERGEN_RUSTC_CHANNEL=stable;VERGEN_RUSTC_COMMIT_DATE=2021-05-09;VERGEN_RUSTC_COMMIT_HASH=9bc8c42bb2f19e745a63f3445f1ac248fb015e53;VERGEN_RUSTC_HOST_TRIPLE=x86_64-apple-darwin;VERGEN_RUSTC_LLVM_VERSION=12.0;VERGEN_RUSTC_SEMVER=1.52.1;
```

## Config

```rust
// Emboss a env variable by name
emboss!(FOO_ENV_VAR);

// Includes `VERGEN_BUILD_*`
emboss!(group=build);

// Includes `VERGEN_GIT_*`
emboss!(group=git);

// Includes `VERGEN_RUSTC_*`
emboss!(group=rustc);

// Includes `VERGEN_CARGO_*`
emboss!(group=cargo);

// Includes `VERGEN_SYSINFO_*`
emboss!(group=sysinfo);

// Includes the following environment variables:
//  VERGEN_BUILD_TIMESTAMP
//  VERGEN_BUILD_SEMVER
//  VERGEN_RUSTC_SEMVER 
//  VERGEN_CARGO_PROFILE 
//  VERGEN_CARGO_FEATURES 
// Which rsps can use to display detailed information about your binary when it runs
emboss!(group=rsps);

// An alias for the above
emboss!();

// You can also specify multiple groups at once
// This will include both `VERGEN_SYSINFO_*` and `VERGEN_GIT_*`
emboss!(groups=sysinfo,git);
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

[i-emboss]: ./EMBOSS.jpg
[vergen]: https://github.com/rustyhorde/vergen
[rsps]: https://github.com/mbStavola/rsps
[env-macro-limitation]: https://github.com/rust-lang/rust/issues/48952
[apache-license]: ./LICENSE-APACHE
[mit-license]: ./LICENSE-MIT