use std::{slice::Iter, collections::{HashSet, BTreeSet}};

use crate::{Rule, Symbol}; 


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RuleId {
    id: usize
}

impl From<usize> for RuleId {
    fn from(id: usize) -> Self {
        RuleId { id }
    }
}

#[derive(Debug, Clone)]
pub(super) struct Grammar<'a> {
    rules: BTreeSet<Rule<'a>>,
    terminals: BTreeSet<Symbol<'a>>,
    non_terminals: Vec<Symbol<'a>>,
}

impl<'a> Grammar<'a> {
    pub(super) fn new() -> Grammar<'a> {
        todo!()
    }
    // pub(super) fn iter_by_symbol<'b>(&'b self, head: &Symbol) -> GrammarIter<'b, 'a> {
    //     todo!()
    // }
    // pub(super) fn first_sets<'a>(&self, remainder: &'a [&'a Symbol]) -> First {
    //     todo!()
    // }

    pub(super) fn terminals(&self) -> &[Symbol] {
        todo!()
    }

    pub(super) fn nonterminals(&self) -> &[Symbol] {
        todo!()
    }

    pub(super) fn iter_by_rule_id(&self, symbol: Symbol) -> impl Iterator<Item = &Rule> {
        todo!()
    }


}

// pub(super) struct GrammarIter<'b, 'a> {
//     rules: Iter<'b, &'b Rule<'a>>
// }

// impl<'b, 'a> Iterator for GrammarIter<'a, 'b> {
//     type Item = &'a Rule<'b>;

//     fn next(&mut self) -> Option<&'a Rule<'b>> {
//         todo!()
//     }
// }