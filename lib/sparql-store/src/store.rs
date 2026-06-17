// This is free and unencumbered software released into the public domain.

use rdf_store::Store;

/// A SPARQL store that supports R/O and R/W transactions.
pub struct SparqlStore<T: Store> {
    inner: T,
}

impl<T: Store> SparqlStore<T> {
    /// Wraps an RDF quad store for SPARQL compatibility.
    pub fn new(inner: impl Into<T>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}

impl<T: Store> Store for SparqlStore<T> {
    type Error = T::Error;
    type Read = T::Read;
    type Write = T::Write;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        Ok(self.inner.read().await?)
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        Ok(self.inner.write().await?)
    }
}
