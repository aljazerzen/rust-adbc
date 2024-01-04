# Rust bindings for Apache ADBC

> Status: abandoned due to too high packaging cost

Rust crate with bindings to pre-built libraries of Apache [Arrow Database Connectivity (ADBC)](https://arrow.apache.org/adbc/current/index.html).

Libraries are built using Nix flakes [arrow-adbc-nix](https://github.com/aljazerzen/arrow-adbc-nix) and [arrow-nanoarrow-nix](https://github.com/aljazerzen/arrow-nanoarrow-nix).

## Status

TL DR; Rust version of ADBC is [ConnectorX](https://docs.rs/connectorx/)

I've abandoned this effort because I've realised that the roadmap of this effort looks bleak.

This wrapper:
- Is [not ergonomic to use](./libadbc-driver-postgresql-sys/tests/test.rs), since it needs to interact with C/C++ library over FFI.
- Requires pre-built libraries and their transitive dependencies. This means it cannot be published to crates.io, since it would not build without them.
- Would need a new wrapper Rust API to be useful.

Such API has been defined in [the official ADBC repo](https://github.com/apache/arrow-adbc/blob/main/rust/src/lib.rs) using Rust traits.

An alternative to this effort would be a pure Rust implementation of the official Rust ADBC API.
It would use existing Rust libraries for driver implementation and would not require ADBC driver manager and dynamic loading to use multiple drivers.

Such pure-Rust implementation already exists, but not under name "ADBC", but "[ConnectorX](https://docs.rs/connectorx/)".
To get it under ADBC name, one could implement the ADBC API using ConnectorX,
but I'm not sure if that would have any benefits over using ConnectorX directly.
