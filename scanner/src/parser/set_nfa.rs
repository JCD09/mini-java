use std::{collections::{HashSet, HashMap, BTreeSet, VecDeque}, todo, fmt::Display, writeln};

use fnv::FnvHashMap;
use indexmap::IndexSet;

use super::Nfa;

const CAPACITY: usize = 5;


// A second attempt to build a Thompson Nfa. This time I used 
// use indicies instead of references. Testing appears to be easier. However, when implementing 
// regeular operations indicies have to be adjusted, which introduces overhead. 
// It is unlikelt that regular expression will exceed that range. 


// I am very inclined to think that alterantive approach would be to use unsafe rust. 

#[derive(Debug)]
pub struct IndexNfa {
    initial_state: i64, 
    accepting_state: i64, 
    states: FnvHashMap<i64, State> 
                
}

impl IndexNfa {
    fn link_with(&mut self, other: i64) {
        if let Some(state) = self.states.get_mut(&self.accepting_state) {
            state.epsilon(other);
        }
    }

    fn link_with_self(&mut self, other: i64) {
        if let Some(state) = self.states.get_mut(&self.accepting_state) {
            state.weak(other,self.initial_state);
        } 
    }

    // this is function for testing purposes. Each nfa is traversed in specific DFS way and keeps track of the visited states. 
    // At the end expected and actual sequences are compared; 
    fn traverse(&mut self) -> IndexSet<i64> {
        let mut output = IndexSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.initial_state);
        while let Some(state_id) = queue.pop_back() {
            if let Some(state) = self.states.get(&state_id) {
                if !output.contains(&state.id()) {
                    match state {
                        State::Character { id, next, transition } => {
                            output.insert(*id);
                            queue.push_back(*next);
                        },
                        State::DoubleEpsilon { id, first, second } => {
                            output.insert(*id);
                            queue.push_back(*second);
                            queue.push_back(*first);
                        },
                        State::Weak { id, first, weak } => {
                            output.insert(*id);
                            queue.push_back(*first);
                            // try weak later
                        },
                        State::Terminal { id } => {
                            output.insert(*id);
                        },
                        State::Epsilon { id, next } => {
                            output.insert(*id);
                            queue.push_back(*next);
                        },
                    }
                } 
            };
        }

        output
    }
}


impl Display for IndexNfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NFA: {{")?; 
        writeln!(f, "     initial state: {}", self.initial_state)?;
        writeln!(f, "     accepting state: {}", self.accepting_state)?;
        self.states.iter().for_each(|f1| {
            write!(f, "{} {}",f1.0, f1.1);
         
        });
        Ok(())      
    }
}

#[derive(Debug)]
enum State {
    Character {
        id: i64,
        next: i64,
        transition: char
    },
    DoubleEpsilon {
        id: i64,
        first: i64, 
        second: i64, 
    },
    Weak {
        id: i64,
        first: i64, 
        weak: i64, 
    },
    Terminal {
        id: i64,
    },
    Epsilon {
        id: i64,
        next: i64
    }
}

impl State {
    fn apply_offset(&mut self, offset: i64) {
        match self {
            State::Character { 
                id: id, 
                next, 
                transition 
            } => {
                *id += offset;
                *next += offset;
            }
            State::DoubleEpsilon { 
                id, 
                first, 
                second 
            } => {
                *id += offset; 
                *first += offset;
                *second += offset; 
            },
            State::Weak { 
                id, 
                first, 
                weak 
            } => {
                *id += offset; 
                *first += offset; 
                *weak += offset; 
            },
            State::Terminal { 
                id 
            } => {
                *id += offset;
            },
            State::Epsilon { 
                id, 
                next 
            } => {
                *id += offset; 
                *next += offset;
            },
        }
    }

    fn epsilon(&mut self, other: i64) {

        if let State::Terminal { id } = self {
            *self = State::Epsilon { 
                id: *id,
                next: other
            };
           
        }
    }

    fn weak(&mut self, other: i64, weak: i64) {

        if let State::Terminal { id } = self {
            *self = State::Weak { 
                id: *id,
                first: other,
                weak
            };
           
        }
    }
    fn id(&self) -> i64 {
        match  self {
            State::Character { id, next, transition } => *id,
            State::DoubleEpsilon { id, first, second } => *id,
            State::Weak { id, first, weak } => *id,
            State::Terminal { id } => *id,
            State::Epsilon { id, next } => *id,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Character { id: id, next, transition } => {
                writeln!(f, "       Character: {{")?; 
                writeln!(f, "           id: {}", id)?;
                writeln!(f, "           transition: {}", transition)?;
                writeln!(f, "           next: {}", next)?;
                writeln!(f, "       }}")?; 
            },
            State::DoubleEpsilon { id, first, second } => {
                writeln!(f, "       Double Epsilon: {{")?; 
                writeln!(f, "           id: {}", id)?;
                writeln!(f, "           top: {}", first)?;
                writeln!(f, "           bottom: {}", second)?;
                writeln!(f, "       }}")?; 
            },
            State::Weak { id, first, weak } => {
                writeln!(f, "       Weak: {{")?; 
                writeln!(f, "           id: {}", id)?;
                writeln!(f, "           first: {}", first)?;
                writeln!(f, "           Weak: {}", weak)?;
                writeln!(f, "       }}")?; 
            },
            State::Terminal { id } => {
                writeln!(f, "       Terminal: {{")?; 
                writeln!(f, "           id: {}", id)?;
                writeln!(f, "       }}")?; 
            },
            State::Epsilon { id, next } => {
                writeln!(f, "       Epsilon: {{")?; 
                writeln!(f, "           id: {}", id)?;
                writeln!(f, "           next: {}", next)?;
                writeln!(f, "       }}")?; 
            },
        }

        Ok(())
    }
}

// index manipulations is still tedious but better compare to references. 
impl Nfa for IndexNfa {
    
    fn with_epsilon_transition() -> Self {
        todo!()
    }

    fn with_character_transition(transition_symbol: char) -> Self {
        let mut map = FnvHashMap::default();
        let initial_state = State::Character { 
            id: 0,
            next: 1, 
            transition: transition_symbol 
        };
        map.insert(0, initial_state);

        let accepting_state = State::Terminal {
            id: 1, 
        };
        map.insert(1, accepting_state);
        
        IndexNfa { 
            initial_state: 0, 
            accepting_state: 1, 
            states: map 
        }
    }

    fn concatenation(mut left: Self, mut right: Self) -> Self {
        let offset = left.accepting_state - right.initial_state + 1; 

        right.states.drain().for_each(|(k,mut node)|{
            node.apply_offset(offset);
            left.states.insert(k+offset, node);
        });

        left.link_with(right.initial_state + offset);
        left.accepting_state = right.accepting_state + offset;
        left
    }

    fn union(mut top: Self, mut bottom: Self) -> Self {
        let offset = top.accepting_state - bottom.initial_state + 1;

        let new_initial_state = State::DoubleEpsilon { 
            id: top.initial_state - 1, 
            first: top.initial_state, 
            second: bottom.initial_state + offset 
        };

        let new_accepting_state = State::Terminal { 
            id: bottom.accepting_state + offset + 1
        };

        top.link_with(bottom.accepting_state + offset + 1);
        bottom.link_with(bottom.accepting_state);

        bottom.states.drain().for_each(|(k,mut node)|{
            node.apply_offset(offset);
            top.states.insert(k + offset, node);
        });

        top.states.insert(top.initial_state - 1, new_initial_state);
        top.states.insert(bottom.accepting_state + offset + 1, new_accepting_state);
        top.initial_state += -1; 
        top.accepting_state = bottom.accepting_state + offset + 1;
        top
    }

    fn kleene_star(mut nfa: Self) -> Self {

        let new_accepting_state = State::Terminal { 
            id: nfa.accepting_state + 1
        };

        let new_initial_state = State::DoubleEpsilon { 
            id: nfa.initial_state - 1, 
            first: nfa.initial_state, 
            second: nfa.accepting_state + 1
        };

        nfa.link_with_self(nfa.accepting_state + 1);
        nfa.link_with(nfa.accepting_state + 1);

        nfa.states.insert(nfa.accepting_state + 1, new_accepting_state);
        nfa.states.insert(nfa.initial_state - 1, new_initial_state);

        nfa.initial_state -= 1;
        nfa.accepting_state += 1;

        nfa 
    }

    fn kleene_plus(nfa: Self) -> Self {
        todo!()
    }

    fn kleene_question(nfa: Self) -> Self {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use std::{println, result};

    use indexmap::{IndexMap, IndexSet};

    use crate::parser::{Nfa};

    use super::IndexNfa;


    #[test]
    fn test() {

        let a: IndexNfa = Nfa::with_character_transition('a');
        // println!("{}", &a);
        let b: IndexNfa = Nfa::with_character_transition('b');

        let c: IndexNfa = Nfa::with_character_transition('c');
        // println!("{}", &b);
        let mut d = Nfa::union(a, b);
        // let mut f = Nfa::union(d, c);

    }

    #[test]
    fn character_transitio_test() {
        let mut c = IndexNfa::with_character_transition('a');
        let result = IndexSet::from([
            0,1
        ]);
        assert_eq!(c.traverse(),result);
    }

    #[test]
    fn concatenation_test() {
        let mut nfa_a = IndexNfa::with_character_transition('a');
        let mut nfa_b = IndexNfa::with_character_transition('b');
        let mut nfa_r = IndexNfa::concatenation(nfa_a, nfa_b); 
        let result = IndexSet::from([
            0,1,2,3
        ]);
        assert_eq!(nfa_r.traverse(),result);
    }
}