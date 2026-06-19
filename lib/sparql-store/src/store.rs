// This is free and unencumbered software released into the public domain.

use crate::{SparqlRead, SparqlWrite};
use rdf_store::Store;
use tokio::runtime::Handle;

/// A SPARQL store that supports R/O and R/W transactions.
#[derive(Debug)]
pub struct SparqlStore<T: Store> {
    pub(crate) inner: T,
    pub(crate) handle: Handle,
}

impl<T: Store> SparqlStore<T> {
    /// Wraps an RDF quad store for SPARQL compatibility.
    pub fn new(inner: impl Into<T>, handle: Handle) -> Self {
        Self {
            inner: inner.into(),
            handle,
        }
    }

    pub async fn read(&mut self) -> Result<SparqlRead<T::Read>, T::Error> {
        Ok(SparqlRead::new(
            self.inner.read().await?,
            self.handle.clone(),
        ))
    }

    pub async fn write(&mut self) -> Result<SparqlWrite<T::Write>, T::Error> {
        Ok(SparqlWrite::new(
            self.inner.write().await?,
            self.handle.clone(),
        ))
    }
}

impl<T: Store + Send> Store for SparqlStore<T>
where
    <T as Store>::Read: Sync,
{
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
