// This is free and unencumbered software released into the public domain.

use crate::SparqlStore;
use alloc::sync::Arc;
use oxrdf::Term;
use rdf_model::interop::OxrdfTerm;
use rdf_store::HeapStore;
use spareval::QueryEvaluationError;
use spareval::{InternalQuad, QueryableDataset};

impl<'a> QueryableDataset<'a> for &'a SparqlStore<Arc<HeapStore>> {
    type InternalTerm = OxrdfTerm;
    type Error = QueryEvaluationError;

    fn internal_quads_for_pattern(
        &self,
        _s: Option<&Self::InternalTerm>,
        _p: Option<&Self::InternalTerm>,
        _o: Option<&Self::InternalTerm>,
        _g: Option<Option<&Self::InternalTerm>>,
    ) -> impl Iterator<Item = Result<InternalQuad<Self::InternalTerm>, Self::Error>> + use<'a> {
        core::iter::empty() // TODO
    }

    fn internalize_term(&self, term: Term) -> Result<Self::InternalTerm, Self::Error> {
        Ok(term.into())
    }

    fn externalize_term(&self, term: Self::InternalTerm) -> Result<Term, Self::Error> {
        Ok(term.into_inner())
    }
}
