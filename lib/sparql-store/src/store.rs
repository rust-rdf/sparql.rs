// This is free and unencumbered software released into the public domain.

use crate::{SparqlRead, SparqlWrite};
use rdf_store::Store;

/// A SPARQL store that supports R/O and R/W transactions.
#[derive(Debug)]
pub struct SparqlStore<T: Store>(T);

impl<T: Store> SparqlStore<T> {
    /// Wraps an RDF quad store for SPARQL compatibility.
    pub fn new(inner: impl Into<T>) -> Self {
        Self(inner.into())
    }

    pub async fn read(&mut self) -> Result<SparqlRead<T::Read>, T::Error> {
        Ok(SparqlRead::new(self.0.read().await?))
    }

    pub async fn write(&mut self) -> Result<SparqlWrite<T::Write>, T::Error> {
        Ok(SparqlWrite::new(self.0.write().await?))
    }
}

impl<T: Store> Store for SparqlStore<T> {
    type Error = T::Error;
    type Read = SparqlRead<T::Read>;
    type Write = SparqlWrite<T::Write>;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        self.read().await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        self.write().await
    }
}
