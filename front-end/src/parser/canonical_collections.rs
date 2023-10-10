use std::cell::Cell;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::hash_set::Drain;
use std::collections::hash_set::Iter as HashIter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;
use std::slice::Iter;
use std::slice::IterMut;

use crate::Rule;
use crate::Symbol;
use crate::utils::DFA;

use super::grammar::Grammar;
use super::item::Item;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct CCStateId {
    id: usize
}

impl From<usize> for CCStateId {
    fn from(id: usize) -> Self {
        CCStateId { id }
    }
}
/////////////////////////////////////////////////////////////////////////////////
/// 
/// Canonical Collection 
/// 
/////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub(super) struct CanonicalCollection<'a, 'b> 
where 'a: 'b 
{
    items: BTreeSet<Item<'a, 'b>>
}

impl<'a, 'b> CanonicalCollection<'a, 'b> 
where 'a: 'b
{
    fn new() -> CanonicalCollection<'a, 'b> {
        CanonicalCollection { 
            items: BTreeSet::new(),
        }
    }

    fn insert(&mut self, item: Item<'a, 'b>) {
        self.items.insert(item);
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &Item<'a, 'b>> {
        self.items.iter()
    }
}

// pub(super) struct CCIter<'a, 'b, 'c> {
//     items: Vec<&'c Item<'a, 'b>>,
//     index: usize,
//     capacity: usize
// }

// impl<'a, 'b, 'c> Iterator for CCIter<'a, 'b, 'c> {
//     type Item = &'c Item<'a, 'b>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let index = self.index; 
//         self.index += 1;
//         if index < self.capacity {
//             return Some(&self.items[index]);
//         }
//         None
//     }
// }

/////////////////////////////////////////////////////////////////////////////////
/// 
///                   CC Calculations        
/// 
/////////////////////////////////////////////////////////////////////////////////
pub(super) fn canonical_collections<'a, 'b>(grammar: &'b Grammar<'a>) -> DFA<CanonicalCollection<'a, 'b>, Symbol<'a>> {

    let mut start_set = CanonicalCollection::new();
    for rule in grammar.iter_by_rule_id(Symbol::Goal) {
        // start_set.insert(Item::new(rule,&Symbol::EndOfFile));
    }
    // start_set = closure(grammar, start_set);
    // let mut dfa = DFA::new(start_set);
    // let mut unmarked_sets = HashSet::new();
    // unmarked_sets.insert(0);

    // loop {
    //     // let mut updated = false;
    //     // let mut sets = unmarked_sets.clone();
    //     // unmarked_sets.clear();
    //     // for index in sets.drain() {
    //     //     let unmarked_set = dfa.state_at(index);
    //     //     let from = index;
    //     //     for item in unmarked_set.iter() {
    //     //         if let Some(transition_symbol) = item.symbol_at_placeholder() {
    //     //             let new_set = goto(grammar, &unmarked_set, transition_symbol);
    //     //             if let Some(to) = dfa.index_of(&new_set) {
    //     //                 dfa.add_transition(from, to);
    //     //             } else {
    //     //                 let to = dfa.add_state(new_set);
    //     //                 dfa.add_transition(from, to);
    //     //                 unmarked_sets.insert(to);
    //     //                 updated = true; 
    //     //             }
    //     //         }
    //     //     }
    //     // }
    //     // if updated == false {
    //     //     break;
    //     // }
    // }
    todo!()

}

// fn start_set<'a>(grammar: &'a Grammar) -> CanonicalCollection<'a> {
//     let mut start_set = CanonicalCollection::new();
//     for rule in grammar.iter_by_id(&Symbol::Goal) {
//         start_set.insert(Item::new(rule,&Symbol::EndOfFile));
//     }
//     closure(grammar, start_set)
// } 


// pub(super) fn closure<'a>(grammar: &'a Grammar, cc: CanonicalCollection<'a>) -> CanonicalCollection<'a> {
//     todo!()
// }

// pub(super) fn goto<'a>(grammar: &'a Grammar, canonical_collection: &CanonicalCollection<'a>, transition_symbol: &'a Symbol) -> CanonicalCollection<'a> {
//     todo!()
// }

#[cfg(test)]
mod test {
    use std::{collections::{HashSet, HashMap}, mem, cell::{RefCell, Cell}, borrow::BorrowMut, rc::Rc, hash};

    use crate::private::symbol::Symbol;

    struct Test {
        a: usize,
        b: usize,
        c: String,
    }

    fn func(test: Test) -> Test {
        test
    }

    #[test]
    fn test() {
        let a:HashSet<usize> = HashSet::new();
        let v:HashSet<Box<usize>> = HashSet::new();
        let c:Vec<usize> = Vec::new();
 
        println!("abcd: {}", mem::size_of::<HashSet<usize>>());
        println!("abcd: {}", mem::size_of::<&HashSet<usize>>());
    }

    #[test]
    fn test2() {
        let mut hashset = HashSet::<usize>::new();
        for i in 0..10 {
            hashset.insert(i);
        }

        let mut references = Vec::new();
        for item in hashset.iter() {
            references.push(Rc::new(*item));
        }

        for r in references.iter() {
            hashset.insert(12);
        }
    }

    #[test]
    fn test3() {
        let v = RefCell::new(vec![1, 2, 3]);

        let r1 = &v;           // Immutable reference to v
        let r2 = &v;           // Another immutable reference to v
        let mut r3 = v.borrow_mut();       // Mutable reference to v
        r3.push(4);
        println!("{:?}", r1);  // Ok
        println!("{:?}", r2);  // Ok
        println!("{:?}", r3);  // Not ok, will not compile

    }
}

