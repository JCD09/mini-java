
use std::ops::Index;
use std::hash::Hash;

use super::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub(crate) struct Rule<'a> {
    head: Symbol<'a>,
    body: Vec<Symbol<'a>>
}

impl<'a> Rule<'a> {
    pub(crate) fn new(head: Symbol<'a>, body: Vec<Symbol<'a>>) -> Rule<'a> {
        Rule { 
            head, 
            body 
        }
    }

    pub(crate) fn head(&self) -> &Symbol<'a> {
        &self.head
    }
    
    pub(crate) fn body(&self) -> &[Symbol<'a>] {
        &self.body
    }
    
}

