use core::num;
use std::collections::{HashMap, BTreeSet};
use std::ops::{Index, IndexMut};
use std::hash::Hash;

use super::canonical_collections::{CanonicalCollection, CCStateId};
use super::grammar::Grammar;
use super::grammar::RuleId;

use crate::Symbol;
use crate::utils::{Table, DFA};


pub(super) fn fill_tables<'context, 'item, 'grammar>(
    grammar: &'item Grammar<'context>, 
    canonical_collections: &'grammar DFA<CanonicalCollection<'context, 'item>, Symbol<'context>>
) -> (Table<CCStateId, Symbol<'grammar>, Action>, Table<CCStateId, Symbol<'grammar>, CCStateId>) 
where 
    'context: 'item,
    'item: 'grammar
{
    let capacity = canonical_collections.number_of_states();
    let mut action_table = Table::<CCStateId, Symbol, Action>::new(capacity);
    let mut goto_table = Table::<CCStateId, Symbol, CCStateId>::new(capacity);

    for (state_id, canonical_collection) in canonical_collections.enumerate_states() {
        for item in canonical_collection.iter() {
            if let Some(symbol) = item.symbol_at_placeholder() {
                if let Some(goto_state ) = canonical_collections.transition_from_on(canonical_collection, symbol) {
                    let goto_state_id = canonical_collections.lookup_state_id(goto_state); 
                    action_table.insert(state_id.into(), symbol, Action::Shift(goto_state_id.into()));
                }         
            } else {
                match item.lookahead() {
                    Symbol::EndOfFile => {
                        action_table.insert(state_id.into(), Symbol::EndOfFile, Action::Accept);
                    }
                    lookahead@_ => {
                        let rule_id = item.rule_id().into();
                        action_table.insert(state_id.into(), lookahead, Action::Reduce(rule_id));
                    }
                }
            }
        }

        for non_terminal in grammar.nonterminals() {
            if let Some(goto_state) = canonical_collections.transition_from_on(canonical_collection, *non_terminal) {
                let goto_state = canonical_collections.lookup_state_id(goto_state);
                goto_table.insert(state_id.into(), *non_terminal, goto_state.into());
            }
        }

    }
    
    (action_table, goto_table)

}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Action {
    Accept, 
    Shift(CCStateId),
    Reduce(RuleId)
}
