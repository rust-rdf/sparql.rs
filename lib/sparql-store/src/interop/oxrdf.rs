// This is free and unencumbered software released into the public domain.

use crate::SparqlRead;
use alloc::{sync::Arc, vec::Vec};
use oxrdf::Term;
use rdf_model::{QuadPattern, Statement};
use rdf_store::ReadTransaction;
use spareval::{InternalQuad, QueryEvaluationError, QueryableDataset};

impl<'a, T: ReadTransaction + Send + Sync + 'static> QueryableDataset<'a> for &'a SparqlRead<T>
where
    <T as ReadTransaction>::Term: From<Term> + Into<Term>,
{
    type InternalTerm = Term;
    type Error = QueryEvaluationError;

    /// Returns an iterator over the internal quads that match the given pattern.
    fn internal_quads_for_pattern(
        &self,
        s: Option<&Self::InternalTerm>,
        p: Option<&Self::InternalTerm>,
        o: Option<&Self::InternalTerm>,
        _g: Option<Option<&Self::InternalTerm>>,
    ) -> impl Iterator<Item = Result<InternalQuad<Self::InternalTerm>, Self::Error>> + use<'a, T>
    {
        let pattern = QuadPattern::<<T as ReadTransaction>::Term>::new(
            s.cloned().map(Into::into),
            p.cloned().map(Into::into),
            o.cloned().map(Into::into),
            None, // TODO: _g
        );
        let inner = Arc::clone(&self.inner);
        let handle = self.handle.clone();
        let output = tokio::task::block_in_place(move || {
            handle.block_on(async move {
                use futures::StreamExt;
                let output: Vec<Result<InternalQuad<Self::InternalTerm>, Self::Error>> = inner
                    .r#match(pattern)
                    .map(|quad_result| match quad_result {
                        Ok(quad) => {
                            let (s, p, o, g) = quad.to_quad().into_inner();
                            Ok(InternalQuad {
                                subject: s.into(),
                                predicate: p.into(),
                                object: o.into(),
                                graph_name: g.map(|g| g.into()),
                            })
                        },
                        Err(_) => Err(QueryEvaluationError::InvalidStorageTripleTerm), // TODO
                    })
                    .collect::<Vec<_>>()
                    .await;
                output
            })
        });
        output.into_iter() // TODO: wrap the stream into an iterator directly
    }

    /// Converts an `oxrdf::Term` to the internal term type.
    fn internalize_term(&self, term: Term) -> Result<Self::InternalTerm, Self::Error> {
        Ok(term.into())
    }

    /// Converts an internal term to an `oxrdf::Term`.
    fn externalize_term(&self, term: Self::InternalTerm) -> Result<Term, Self::Error> {
        Ok(term.into())
    }
}
