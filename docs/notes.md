# Notes

## Compile time error - linker missing
Also, some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one now.

## Rust prelude
Module std::prelude

The Rust Prelude.

Rust comes with a variety of things in its standard library. However, if you had to manually import every single thing that you used, it would be very verbose.

On a technical level, Rust inserts

`extern crate std;Run`

into the crate root of every crate, and

`use std::prelude::v1::*;Run`
