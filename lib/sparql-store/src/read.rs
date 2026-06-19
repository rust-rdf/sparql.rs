// This is free and unencumbered software released into the public domain.

use alloc::sync::Arc;
use futures::Stream;
use rdf_store::ReadTransaction;
use tokio::runtime::Handle;

/// A read-only (R/O) transaction wrapper for SPARQL compatibility.
#[derive(Debug)]
pub struct SparqlRead<T: ReadTransaction + Send> {
    pub(crate) inner: Arc<T>,
    #[allow(dead_code)]
    pub(crate) handle: Handle,
}

impl<T: ReadTransaction + Send> SparqlRead<T> {
    /// Wraps an RDF store transaction for SPARQL compatibility.
    pub fn new(inner: T, handle: Handle) -> Self {
        Self {
            inner: Arc::new(inner),
            handle,
        }
    }
}

impl<T: ReadTransaction + Send> ReadTransaction for SparqlRead<T> {
    type Error = T::Error;
    type Term = T::Term;
    type Statement = T::Statement;
    type StatementPattern = T::StatementPattern;

    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> + Send {
        self.inner.contexts()
    }

    fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        self.inner.contains(pattern)
    }

    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<u64, Self::Error>> + Send {
        self.inner.count(pattern)
    }

    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        self.inner.r#match(pattern)
    }
}
