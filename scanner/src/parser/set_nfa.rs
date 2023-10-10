use std::{collections::{HashSet, HashMap}, todo, fmt::Display, writeln};

use fnv::FnvHashMap;

use super::NfaBuilder;

const CAPACITY: usize = 5;


// A second attempt to build a Thompson Nfa. This time I used 
// use indicies instead of references. Testing appears to be easier. However, when implementing 
// regeular operations indicies have to be adjusted, when two maps are merged, which introduces overhead. 
// On the other hand code is easier to write. 

// The range for signed itneger is -9,223,372,036,854,775,808 and a maximum value of 9,223,372,036,854,775,807
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
        node_id: i64,
        next: i64,
        transition: char
    },
    DoubleEpsilon {
        node_id: i64,
        first: i64, 
        second: i64, 
    },
    Weak {
        node_id: i64,
        first: i64, 
        weak: i64, 
    },
    Terminal {
        node_id: i64,
    },
    Epsilon {
        node_id: i64,
        next: i64
    }
}

impl State {
    fn apply_offset(&mut self, offset: i64) {
        match self {
            State::Character { 
                node_id, 
                next, 
                transition 
            } => {
                *node_id += offset;
                *next += offset;
            }
            State::DoubleEpsilon { 
                node_id, 
                first, 
                second 
            } => {
                *node_id += offset; 
                *first += offset;
                *second += offset; 
            },
            State::Weak { 
                node_id, 
                first, 
                weak 
            } => {
                *node_id += offset; 
                *first += offset; 
                *weak += offset; 
            },
            State::Terminal { 
                node_id 
            } => {
                *node_id += offset;
            },
            State::Epsilon { 
                node_id, 
                next 
            } => {
                *node_id += offset; 
                *next += offset;
            },
        }
    }

    fn epsilon(&mut self, other: i64) {
        if let State::Terminal { node_id } = self {
            *self = State::Epsilon { 
                node_id: *node_id,
                next: other
            }
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Character { node_id, next, transition } => {
                writeln!(f, "       Character: {{")?; 
                writeln!(f, "           node_id: {}", node_id)?;
                writeln!(f, "           transition: {}", transition)?;
                writeln!(f, "           next: {}", next)?;
                writeln!(f, "       }}")?; 
            },
            State::DoubleEpsilon { node_id, first, second } => {},
            State::Weak { node_id, first, weak } => todo!(),
            State::Terminal { node_id } => {
                writeln!(f, "       Terminal: {{")?; 
                writeln!(f, "           node_id: {}", node_id)?;
                writeln!(f, "       }}")?; 
            },
            State::Epsilon { node_id, next } => {
                writeln!(f, "       Epsilon: {{")?; 
                writeln!(f, "           node_id: {}", node_id)?;
                writeln!(f, "           next: {}", next)?;
                writeln!(f, "       }}")?; 
            },
        }

        Ok(())
    }
}

// index manipulations is still tedious. 
impl NfaBuilder for IndexNfa {
    fn with_epsilon_transition() -> Self {
        todo!()
    }
    fn with_character_transition(transition_symbol: char) -> Self {
        let mut map = FnvHashMap::default();
        let initial_state = State::Character { 
            node_id: 0,
            next: 1, 
            transition: transition_symbol 
        };
        map.insert(0, initial_state);

        let accepting_state = State::Terminal {
            node_id: 1, 
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

        bottom.states.drain().for_each(|(k,mut node)|{
            node.apply_offset(offset);
            top.states.insert(k+offset, node);
        });

        let initial_state = State::DoubleEpsilon { 
            node_id: top.initial_state - 1, 
            first: top.initial_state, 
            second: bottom.initial_state + offset 
        };

        // let accepting_state = State::Terminal { node_id: () } { 
        //     node_id: bottom.accepting_state + offset + 1, 
        //     first: , 
        //     second: () 
        // }


        todo!()
    }

    fn kleene_star(nfa: Self) -> Self {
        todo!()
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
    use std::println;

    use crate::parser::{NfaBuilder, Nfa};

    use super::IndexNfa;


    #[test]
    fn test() {

        let a: IndexNfa = NfaBuilder::with_character_transition('a');
        // println!("{}", &a);
        let b: IndexNfa = NfaBuilder::with_character_transition('b');
        // println!("{}", &b);
        let c = NfaBuilder::concatenation(a, b);

        println!("{}", &c);

    }
}