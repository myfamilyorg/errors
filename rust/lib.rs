#![no_std]

extern crate error;

use error::prelude::*;

errors!(
    Alloc,
    IllegalArgument,
    OutOfBounds,
    MisalignedPointer,
    IllegalState,
    Todo
);
