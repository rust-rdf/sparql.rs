// This is free and unencumbered software released into the public domain.

use super::{SyntaxError, Unit};
use spargebra::{Query, SparqlParser, Update};

pub fn parse(input: impl AsRef<str>) -> Result<Unit, SyntaxError> {
    let input = input.as_ref();
    parser()
        .parse_query(input)
        .map(Unit::Query)
        .or_else(|_| parser().parse_update(input).map(Unit::Update))
}

pub fn parse_query(input: impl AsRef<str>) -> Result<Query, SyntaxError> {
    parser().parse_query(input.as_ref())
}

pub fn parse_update(input: impl AsRef<str>) -> Result<Update, SyntaxError> {
    parser().parse_update(input.as_ref())
}

fn parser() -> SparqlParser {
    SparqlParser::new()
}
