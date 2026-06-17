// This is free and unencumbered software released into the public domain.

use rdf_store::WriteTransaction;

/// A read-write (R/W) transaction wrapper for SPARQL compatibility.
#[allow(type_alias_bounds)]
pub type SparqlWrite<T: WriteTransaction> = T;
