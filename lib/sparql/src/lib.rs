// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
//#![allow(unused)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "algebra")]
pub use sparql_algebra as algebra;

#[cfg(feature = "derive")]
pub use sparql_derive as derive;

#[cfg(feature = "engine")]
pub use sparql_engine as engine;

#[cfg(feature = "parser")]
pub use sparql_parser as parser;

#[cfg(feature = "parser")]
pub use sparql_parser::{parse, parse_query, parse_update};

//#[cfg(feature = "store")]
//pub use sparql_store as store;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
