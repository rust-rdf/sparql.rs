// This is free and unencumbered software released into the public domain.

use rdf_store::WriteTransaction;
use tokio::runtime::Handle;

/// A read-write (R/W) transaction wrapper for SPARQL compatibility.
#[derive(Debug)]
pub struct SparqlWrite<T: WriteTransaction + Send> {
    pub(crate) inner: T,
    pub(crate) handle: Handle,
}

impl<T: WriteTransaction + Send> SparqlWrite<T> {
    /// Wraps an RDF store transaction for SPARQL compatibility.
    pub fn new(inner: T, handle: Handle) -> Self {
        Self { inner, handle }
    }
}

impl<T: WriteTransaction + Send> WriteTransaction for SparqlWrite<T> {
    type Error = T::Error;
    type Term = T::Term;
    type Statement = T::Statement;
    type StatementPattern = T::StatementPattern;

    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>> {
        self.inner.rollback()
    }

    fn commit(self) -> impl Future<Output = Result<(), Self::Error>> {
        self.inner.commit()
    }

    fn clear(&mut self) -> impl Future<Output = Result<(), Self::Error>> {
        self.inner.clear()
    }

    fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>> {
        self.inner.insert(statement)
    }

    fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>> {
        self.inner.remove(statement)
    }

    fn delete(
        &mut self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>> {
        self.inner.delete(pattern)
    }
}
