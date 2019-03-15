Flatbuffers for Rust
========

[![Build Status](https://travis-ci.org/termoshtt/rust-flatbuffers.svg?branch=master)](https://travis-ci.org/termoshtt/rust-flatbuffers)

Fork of the Rust part of [Official FlatBuffers implementation](https://github.com/google/flatbuffers)

Features
---------
Generate and include generated Rust binding using proc-macro:

```rust
use flatc_gen::flatc_gen;
flatc_gen!("../fbs/addressbook.fbs");
```

See complete example in [flatc-gen-example](./flatc-gen-example)

Limitations
------------
`flatc-gen` strongly depends on the [unstable feature of proc-macro2](https://docs.rs/proc-macro2/0.4.27/proc_macro2/#unstable-features). You need to use nightly compiler and set `RUSTFLAGS='--cfg procmacro2_semver_exempt'` to enable rustc feature or set it using [.cargo/config](./.cargo/config)

```
[build]
rustflags = ["--cfg", "procmacro2_semver_exempt"]
```

Licensing
----------
Original *Flatbuffers* is licensed under the Apache License, Version 2.0. See [LICENSE](https://github.com/google/flatbuffers/blob/master/LICENSE.txt) for the full license text.
