// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
//#![allow(unused)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use spargebra::{Query, Update};
