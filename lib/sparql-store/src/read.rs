// This is free and unencumbered software released into the public domain.

use futures_util::Stream;
use rdf_store::ReadTransaction;

/// A read-only (R/O) transaction wrapper for SPARQL compatibility.
pub struct SparqlRead<T: ReadTransaction>(T);

impl<T: ReadTransaction> SparqlRead<T> {
    /// Wraps an RDF store transaction for SPARQL compatibility.
    pub fn new(inner: impl Into<T>) -> Self {
        Self(inner.into())
    }
}

impl<T: ReadTransaction> ReadTransaction for SparqlRead<T> {
    type Error = T::Error;
    type Term = T::Term;
    type Statement = T::Statement;
    type StatementPattern = T::StatementPattern;

    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> {
        self.0.contexts()
    }

    fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Future<Output = Result<bool, Self::Error>> {
        self.0.contains(pattern)
    }

    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Future<Output = Result<u64, Self::Error>> {
        self.0.count(pattern)
    }

    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        self.0.r#match(pattern)
    }
}
