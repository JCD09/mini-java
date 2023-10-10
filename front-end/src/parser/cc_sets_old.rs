use std::{collections::{HashSet, hash_set::Iter}, hash::Hash};

use crate::{parser::grammar::Grammar, utils::Dfa};
use super::{Rule, grammar, Symbol};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(super) struct Item<'a> {
    rule: &'a Rule,
    placeholder: usize,
    lookahead: &'a Symbol
}

impl<'a> Item<'a> {
    /// returns next symbol after placeholder
    fn new(rule: &Rule) -> Item<'a> {
        todo!()
    } 

    fn nontermial_at_placeholder(&self) -> Option<&'a Symbol> {
        None
    }
    
    fn symbol_at_placeholder(&self) -> Option<&'a Symbol> {
        None
    }

    fn lookahead(&self) -> &'a[&'a Symbol] {
        todo!()
    }

    fn advance_placeholder(&self) -> Item<'a> {
        todo!()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct CcSet<'a> {
    items: HashSet<Item<'a>>
}
impl<'a> CcSet<'a> {
    fn new() -> CcSet<'a> {
        CcSet { 
            items: HashSet::new() 
        }
    }
    fn iter(&self) -> Iter<Item<'a>> {
        self.items.iter()
    }
}

impl<'a> CcSet<'a> {

}

#[derive(Debug, Clone)]
pub(super) struct CcSets<'a> {
    grammar: &'a Grammar,
    dfa: Dfa<CcSet<'a>>
}

impl<'a> CcSets<'a> {
    pub(super) fn new(grammar: &'a Grammar) -> CcSets<'a> {
        let dfa = cc_sets(grammar);
        CcSets { 
            grammar,
            dfa
        }
    }
}

// Algorithm to construct a canonical collection of sets.
// returns dfa 
fn cc_sets<'a>(grammar: &'a Grammar) -> Dfa<CcSet<'a>> {
    let mut start_set = CcSet {
        items: HashSet::new()
    };
    let mut start_set = CcSet::new();
    for rule in grammar.iter_by_id(&Symbol::Goal) {
        let item = Item::new(rule);
        start_set.items.insert(item);
    }
    start_set = closure(grammar, start_set);


    let cc = Dfa::new(start_set);
    
    cc

}

fn closure<'a>(grammar: &'a Grammar, mut cc_set: CcSet<'a>) -> CcSet<'a> {
    loop {
        let mut old = cc_set.clone();
        for item in cc_set.iter() {
            if let Some(next_symbol) = item.nontermial_at_placeholder() {
                let lookahead = item.lookahead();
                for rule in grammar.iter_by_id(&next_symbol) {
                    for symbol in grammar.first(lookahead) { 
                        let item = Item::new(rule);
                        old.items.insert(item);
                    }
                } 
            }
        }
        if old == cc_set {
            break
        }

        cc_set = old; 
    }
    cc_set
}

fn goto<'a>(grammar: &'a Grammar, cc_set: CcSet<'a>, transition_symbol: &'a Symbol) -> CcSet<'a> {
    let mut next_set = CcSet::new();
    for item in cc_set.iter() {
        if let Some(symbol) = item.symbol_at_placeholder() {
            let next_item = item.advance_placeholder();
            next_set.items.insert(next_item);
        }
    }
    return closure(grammar, cc_set)
}




