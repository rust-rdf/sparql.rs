// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod read;
pub use read::*;

mod store;
pub use store::*;

mod write;
pub use write::*;

/// Interoperability with other Rust libraries.
pub mod interop {
    #[cfg(feature = "oxrdf")]
    mod oxrdf;
    #[cfg(feature = "oxrdf")]
    pub use oxrdf::*;
}
