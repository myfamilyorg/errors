#![no_std]

extern crate error;

pub mod prelude {
    pub use super::*;
}

use error::prelude::*;

errors!(
    Alloc,
    IllegalArgument,
    OutOfBounds,
    MisalignedPointer,
    IllegalState,
    Todo
);
