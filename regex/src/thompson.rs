use std::ops::Add;

use crate::utils;
use utils::immutable_graph::Graph;

const INITIAL_CAPACITY: usize = 5; 
const MULTIPLIER: usize = 2;

type StateId = usize; 
type Nfa = (usize, usize);

#[derive(Debug, Clone)]
struct NfaState {
    id: StateId, 
    transitions: Vec<Transition>
}

#[derive(Debug, Clone)]
struct Transition {
    symbol: Option<char>,
    next: StateId
}

impl NfaState {
    fn new(id: StateId) -> NfaState {
        NfaState {
            id,
            transitions: Vec::with_capacity(INITIAL_CAPACITY)
        }
    }
}

pub struct NFA {
    states: Graph<NfaState>
}

impl NFA {
    pub fn new() -> NFA {
        NFA { 
            states: Graph::new()
        }
    }

    pub fn add_state(&mut self) {
        let state_id = self.states.length();
        let state = NfaState::new(state_id);
        self.states.add_node(state);
    }

    pub fn add_transition(&mut self, from: StateId, to: StateId, symbol: Option<char>) {
        if let Some(state) = self.states.get_mut(from) {
            state.transitions.push(Transition { symbol, next: to })
        } 
    }
    // // add espilon transition 


    // // add character transition  
    // pub fn character_transition(&mut self, char: char) -> Nfa {
    //     let from = self.nfa_states.add_node(StateId::Character(char));
    //     let to = self.nfa_states.add_node(StateId::Accept);
    //     self.nfa_states.add_edge(from, to);
    //     (from, to)
    // }

    // // add concatentation
    // pub fn concatenation(&mut self, left_nfa: Nfa, right_nfa: Nfa) -> Nfa {
    //     if let Some(state_id@StateId::Accept) = self.nfa_states.get_mut(left_nfa.1) {
    //         *state_id = StateId::Epsilon;
    //         self.nfa_states.add_edge(left_nfa.1, right_nfa.0);
    //     }
    //     (left_nfa.0, right_nfa.1)
    // }

    // // 
    // pub fn alternation(&mut self, top_nfa: Nfa, bottom_nfa: Nfa) -> Nfa {
    //     let start = self.nfa_states.add_node(StateId::DoubleEpsilon);
    //     self.nfa_states.add_edge(start, top_nfa.0);
    //     self.nfa_states.add_edge(start, bottom_nfa.0);

    //     let end = self.nfa_states.add_node(StateId::Accept);

    //     if let Some(state_id@StateId::Accept) = self.nfa_states.get_mut(top_nfa.1) {
    //         *state_id = StateId::Epsilon;
    //         self.nfa_states.add_edge(top_nfa.1, end);
    //     }

    //     if let Some(state_id@StateId::Accept) = self.nfa_states.get_mut(bottom_nfa.1) {
    //         *state_id = StateId::Epsilon;
    //         self.nfa_states.add_edge(bottom_nfa.1, end);
    //     }

    //     (start, end)
    // }

    // pub fn repetition(&mut self, nfa: Nfa) -> Nfa {
    //     if let Some(state_id@StateId::Accept) = self.nfa_states.get_mut(nfa.1) {
    //         *state_id = StateId::DoubleEpsilon;
    //         self.nfa_states.add_edge(nfa.1, nfa.0);
    //     }

    //     let start = self.nfa_states.add_node(StateId::DoubleEpsilon);
    //     self.nfa_states.add_edge(start, nfa.0);
    //     let end = self.nfa_states.add_node(StateId::Accept);
    //     self.nfa_states.add_edge(start, end);

    //     (start, end)
    // }

    pub fn to_graphviz_string(&self) -> String {
        let mut graph = String::new();
        graph.push_str("digraph finite_state_machine {\n");
        graph.push_str("fontname=\"Helvetica,Arial,sans-serif\"\n");
        graph.push_str("node [fontname=\"Helvetica,Arial,sans-serif\"]\n");
        graph.push_str("edge [fontname=\"Helvetica,Arial,sans-serif\"]\n");
        graph.push_str("rankdir=LR;\n");
        graph.push_str("node [shape = doublecircle]; 0 3 4 8;\n");
        graph.push_str("node [shape = circle];\n");

        todo!()
    }
}
