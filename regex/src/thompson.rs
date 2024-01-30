use crate::utils;
use utils::immutable_graph::Graph;

const INITIAL_CAPACITY: usize = 5; 
const MULTIPLIER: usize = 2;

type NodeId = usize; 
type Nfa = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum StateId {
    // state with a single epsilon transition
    Epsilon, 
    // state with a double epsilon transitions
    DoubleEpsilon,
    // state with a character epsilon transitions
    Character(char),
    Temp
}

pub struct ThompsonNfa {
    nfa_states: Graph<StateId>
}

impl ThompsonNfa {
    pub fn new() -> ThompsonNfa {
        ThompsonNfa { 
            nfa_states: Graph::new()
        }
    }

    // add espilon transition 
    pub fn add_epsilon(&mut self) -> Nfa {
        let from = self.nfa_states.add_node(StateId::Epsilon);
        let to = self.nfa_states.add_node(StateId::Temp);
        self.nfa_states.add_edge(from, to);
        (from, to)
    }

    // add character transition  
    pub fn add_character(&mut self, char: char) -> Nfa {
        let from = self.nfa_states.add_node(StateId::Character(char));
        let to = self.nfa_states.add_node(StateId::Temp);
        self.nfa_states.add_edge(from, to);
        (from, to)
    }

    // add concatentation
    pub fn concatenate(&mut self, left_nfa: Nfa, right_nfa: Nfa) -> Nfa {
        if let Some(state_id@StateId::Temp) = self.nfa_states.get_mut(left_nfa.1) {
            *state_id = StateId::Epsilon;
            self.nfa_states.add_edge(left_nfa.1, right_nfa.0);
        }
        (left_nfa.0, right_nfa.1)
    }

    // 
    pub fn alternate(&mut self, top_nfa: Nfa, bottom_nfa: Nfa) -> Nfa {
        let start = self.nfa_states.add_node(StateId::DoubleEpsilon);
        self.nfa_states.add_edge(start, top_nfa.0);
        self.nfa_states.add_edge(start, bottom_nfa.0);

        let end = self.nfa_states.add_node(StateId::Temp);

        if let Some(state_id@StateId::Temp) = self.nfa_states.get_mut(top_nfa.1) {
            *state_id = StateId::Epsilon;
            self.nfa_states.add_edge(top_nfa.1, end);
        }

        if let Some(state_id@StateId::Temp) = self.nfa_states.get_mut(bottom_nfa.1) {
            *state_id = StateId::Epsilon;
            self.nfa_states.add_edge(bottom_nfa.1, end);
        }

        (start, end)
    }

    pub fn star(&mut self, nfa: Nfa) -> Nfa {
        if let Some(state_id@StateId::Temp) = self.nfa_states.get_mut(nfa.1) {
            *state_id = StateId::DoubleEpsilon;
            self.nfa_states.add_edge(nfa.1, nfa.0);
        }

        let start = self.nfa_states.add_node(StateId::DoubleEpsilon);
        self.nfa_states.add_edge(start, nfa.0);
        let end = self.nfa_states.add_node(StateId::Temp);
        self.nfa_states.add_edge(start, end);

        (start, end)
    }
}
