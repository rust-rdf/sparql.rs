// This is free and unencumbered software released into the public domain.

use super::SyntaxError;
use spargebra::*;

pub fn parse_query(input: impl AsRef<str>) -> Result<Query, SyntaxError> {
    let parser = SparqlParser::new();
    parser.parse_query(input.as_ref())
}

pub fn parse_update(input: impl AsRef<str>) -> Result<Update, SyntaxError> {
    let parser = SparqlParser::new();
    parser.parse_update(input.as_ref())
}
