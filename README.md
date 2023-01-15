# Rust Toolchain Manifest

This is a library which is capable of parsing the v2 Rust toolchain manifest and
doing some basic queries on it. It is in no way offical, and the Rust
toolchain manifest has the ability to change arbitrarily at any time. It was
written simply because I could not find an existing library that did this.

The Rust toolchain manifest format has clearly been extended over time and some
aspects of it, in particular how missing packages and renames are handled, is
quite confusing.  This library is a best-effort attempt to reverse-engineer the
underlying meaning.
