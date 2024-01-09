#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod private {
    pub mod rule; 
    pub mod symbol;
    pub mod grammar;
    pub mod token;
}

mod scanner;

pub struct SparseSet<E> {
    dense: Vec<E>,
    sparse: Vec<Option<usize>>
}


mod parser;
mod utils;

// inner facing api
use private::rule::Rule; 
use private::symbol::Symbol;
use private::grammar::GRAMMAR;
use private::token::*;
