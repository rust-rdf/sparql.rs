// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod store;
pub use store::*;
