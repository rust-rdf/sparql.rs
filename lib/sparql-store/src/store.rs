// This is free and unencumbered software released into the public domain.

use crate::{SparqlRead, SparqlWrite};
use rdf_store::Store;

/// A SPARQL store that supports R/O and R/W transactions.
pub struct SparqlStore<T: Store>(T);

impl<T: Store> SparqlStore<T> {
    /// Wraps an RDF quad store for SPARQL compatibility.
    pub fn new(inner: impl Into<T>) -> Self {
        Self(inner.into())
    }
}

impl<T: Store> Store for SparqlStore<T> {
    type Error = T::Error;
    type Read = SparqlRead<T::Read>;
    type Write = SparqlWrite<T::Write>;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        Ok(SparqlRead::new(self.0.read().await?))
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        Ok(self.0.write().await?)
    }
}
