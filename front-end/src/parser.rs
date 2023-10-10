mod grammar; 
mod canonical_collections; 
mod tables; 
mod item; 


use std::collections::{BTreeSet, VecDeque};

use grammar::Grammar;
use crate::{Rule, scanner::Scanner};

use canonical_collections::CanonicalCollection;

use self::{canonical_collections::canonical_collections, tables::Action};

// push INVALID, INVALID onto the stack
// push <start symbol, s0> onto the stack
// word ← NextWord( ) from scanner
// while (true) do
// state ← state from pair at top of stack
// if Action[state,word] = “reduce A → β” then
// pop |β| pairs from the stack
// state ← state from pair at top of stack
// push A, Goto[state, A]  onto the stack
// else if Action[state,word] = “shift si
// ” then
// push word, si
// onto the stack
// word ← NextWord( )
// else if Action[state,word] = “accept” and word = eof
// then break
// else throw a syntax error
// report success /* executed the “accept” case */

pub fn parse() {

    todo!()

}