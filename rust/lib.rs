#![no_std]

extern crate error;
pub use error::prelude::*;

errors!(
    Alloc,
    IllegalArgument,
    OutOfBounds,
    MisalignedPointer,
    IllegalState,
    Utf8Error,
    Todo,
    Unknown
);
