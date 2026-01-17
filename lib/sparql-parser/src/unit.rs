// This is free and unencumbered software released into the public domain.

use spargebra::{Query, Update};

#[derive(Clone, Debug)]
pub enum Unit {
    Query(Query),
    Update(Update),
}
