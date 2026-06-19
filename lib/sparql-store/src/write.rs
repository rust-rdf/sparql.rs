// This is free and unencumbered software released into the public domain.

use alloc::sync::Arc;
use futures_util::Stream;
use rdf_store::WriteTransaction;

/// A read-write (R/W) transaction wrapper for SPARQL compatibility.
pub struct SparqlWrite<T: WriteTransaction + Send>(pub(crate) T);

impl<T: WriteTransaction + Send> From<T> for SparqlWrite<T> {
    /// Wraps an RDF store transaction for SPARQL compatibility.
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T: WriteTransaction + Send> WriteTransaction for SparqlWrite<T> {
    type Error = T::Error;
    type Term = T::Term;
    type Statement = T::Statement;
    type StatementPattern = T::StatementPattern;

    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>> {
        self.0.rollback()
    }

    fn commit(self) -> impl Future<Output = Result<(), Self::Error>> {
        self.0.commit()
    }

    fn clear(&mut self) -> impl Future<Output = Result<(), Self::Error>> {
        self.0.clear()
    }

    fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>> {
        self.0.insert(statement)
    }

    fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>> {
        self.0.remove(statement)
    }

    fn delete(
        &mut self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>> {
        self.0.delete(pattern)
    }
}
