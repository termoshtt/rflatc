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

Licensing
----------
Original *Flatbuffers* is licensed under the Apache License, Version 2.0. See [LICENSE](https://github.com/google/flatbuffers/blob/master/LICENSE.txt) for the full license text.
