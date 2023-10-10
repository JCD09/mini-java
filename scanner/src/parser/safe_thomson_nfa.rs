use std::{rc::{Rc, Weak}, todo, cell::RefCell, ops::DerefMut, collections::{HashMap, HashSet, BTreeSet, BTreeMap, VecDeque}, print, println};

use super::{NfaBuilder, Nfa};

/// Thompson NFA graph implementation using safe Rust. 
/// 
/// 
/// 
type Link = Rc<RefCell<Node>>;
type WeakLink = Weak<RefCell<Node>>;


// This was a subproject where I tried to learn how to build graph-like data structures
// in safe Rust. 
//
// My first attempet involvedusing only Rc and then getting mutable reference to the 
// subsequent nodes, however after a day of fiddling with the Rc primitives and reading documentation
// I abandoned the idea. It did not work.  
// 
// The second attempt was a bit more successful. The structure Thompson NFa consists of two types of 
// nodes  there is only one
// type of node that has a cycle, so I tried to use a weak pointer. However, when I had to traverse 
// the graph and using HashSet to mark and keep track of visited nodes, Weak pointer stood in a way.  
// At this point the code for nfa traversal was getting too complicated and I felt this is not a good 
// way go around this problem. . 
// 
// At this time. I could had two approaches, I could choose to wrap weak pointer into something kind of st
// struct and implement necessary traits for that struct and learn all I could learn about 
// traits, but at the end decided against it. I had an intuition taht this was not an idiomatic Rust. 

// The second approach involved using Rc instead of weak for a cycle. The problem with this approach 
// was that it was causing memory leaks. So I had to come up with a function that traverses the graph
// and breaks the cycles manually. Luckily the cycels were easy to find, it is just SingleEpsilon node
// with non empty weak field. 

// GOALS AND LESSONS LEARNED. 
// The goal of this approch was to test the feasibility of representing an nfa using Rc<RefCell<Node>>
// graph pattern. 
// Ultimately this is not viable approach becuase one has to manually break cycles which involves 
// traversing the graph and manually break Rc cycles to avoid memory leaks. Which can introduce, 
// potentially large overhead. 
// Another encountered difficulty is testing. To ensure that nodes visited in the right order, 
// I could have added soem node identifier say number and then when graph is traversed returned the 
// nodes in right sequence, but that adds unnecessary complexity. 





#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Node {
    Char {
        transition: char,
        next: Link
    },
    Epsilon {
        next: Option<Link>,
        weak: Option<Link>
    },

    DoubleEpsilon {
        next: Option<Link>,
        next2: Option<Link>,
    },
}

impl Drop for Node {
    fn drop(&mut self) {
        match self {
            Node::Char { transition, next } => {
                println!("dropping char node"); 
                drop(next.borrow_mut());
            },
            Node::Epsilon { next, weak } => {
                *weak = None;  
                println!("dropping epsilon node");
                if let Some(n1) = next {
                    drop(n1.borrow_mut());
                    }

            },
            Node::DoubleEpsilon { next, next2 } => {              
                if let Some(n1) = next {
                drop(n1.borrow_mut());
                }
                if let Some(n2) = next2 {
                    drop(n2.borrow_mut());
                } 
                println!("dropping double epsilon ");   
            },
        }
    }
}

fn break_cycles(node: Rc<RefCell<Node>>) {
    let mut queue = VecDeque::<Link>::new();
    queue.push_back(node);
    while let Some(node) = queue.pop_front() {
        match &mut *node.borrow_mut() {
            Node::Char { transition, next } => {
                queue.push_back(next.clone());
            },
            Node::Epsilon { next, ref mut weak } => {
                if let Some(n) = next {
                    queue.push_back(n.clone());
                }
                *weak = None;

            },
            Node::DoubleEpsilon { next, next2 } => {
                if let Some(n) = next {
                    queue.push_back(n.clone());
                }
                if let Some(m) = next {
                    queue.push_back(m.clone());
                }
            },
        }
    }

}

#[derive(Debug, Clone)]
struct SafeNfa {
    start: Link,
    finish: Link
}

// fn traverse_and_enumerate(nfa: SafeNfa) -> bool {
//     let mut count = 0;
//     let mut enumerate = BTreeMap::<Link,usize>::new();
//     let mut visited = BTreeSet::<Link>::new();
//     let mut bag = VecDeque::<Link>::new();

//     bag.push_back(nfa.start);

//     while let Some(link) = bag.pop_front() {
//         // so we take reference from the bag.
//         //  we mark it as visited and by doing that we put it in visited set. 
//         visited.insert(link.clone());
//         // also add it to the map so it can enumerated. 
//         enumerate.insert(link.clone(), count);
//         count += 1;

//         let ref link = *link.borrow();
//         match link {
//             Node::Char { transition, next } => {
//                 bag.push_back(next.clone());
//             },
//             Node::Epsilon { next, weak } => {
//                 match (next, weak) {
//                     (Some(_), Some()) => {},
//                     (Some(_), None) => {},
//                     ()
//                 }
//             },
//             Node::DoubleEpsilon { next, next2 } => {

//             },
//         }


//         // we also put it in the enumerate map 


//     }

//     // the rough idea is that we keep track visited nodes but we also enumerate 
//     // them for testing purposes. 

//     true
// }

fn traverse(nfa: SafeNfa) -> BTreeSet<Rc<RefCell<Node>>> {
    let set = BTreeSet::<Link>::new();
    todo!()
}

impl NfaBuilder for SafeNfa {
    fn with_epsilon_transition() -> SafeNfa {
        todo!()
    }

    fn with_character_transition(transition_symbol: char) -> SafeNfa {
        let end_node = Rc::new(RefCell::new(Node::Epsilon { 
            next: None, 
            weak: None
        }));

        let start_node = Rc::new(RefCell::new(Node::Char { 
            transition: transition_symbol, 
            next: end_node.clone()
        } 
        )); 

        SafeNfa { 
            start: start_node, 
            finish: end_node
        }
    }

    fn concatenation(left: SafeNfa, right: SafeNfa) -> SafeNfa {
        let ref mut left_final = *left.finish.borrow_mut();
        match left_final {
            Node::Epsilon { next, weak } => {
                *next = Some(right.start);
            }
            _ => {}
        }

        SafeNfa {
            start: left.start,
            finish: right.finish
        }
    }

    fn union(left: SafeNfa, right: SafeNfa) -> SafeNfa {
        let start_node = Rc::new(RefCell::new(Node::DoubleEpsilon { 
            next: Some(left.start), 
            next2: Some(right.start) 
        }));

        let end_node = Rc::new(RefCell::new(Node::Epsilon { 
            next: None, 
            weak: None
        }));

        let ref mut first =  *left.finish.borrow_mut();
        match first {
            Node::Epsilon { next, weak } => {
                *next = Some(end_node.clone())
            }
            _ => {}
        }

        let ref mut secod =  *right.finish.borrow_mut();
        match first {
            Node::Epsilon { next, weak } => {
                *next = Some(end_node.clone())
            }
            _ => {}
        }

        SafeNfa {
            start: start_node,
            finish: end_node,
        }
    }

    // this is the hardest one;
    fn kleene_star(nfa: SafeNfa) -> SafeNfa {

        let new_final = Rc::new(RefCell::new(Node::Epsilon { 
            next: None, 
            weak: None 
        }));

        let ref mut mutable_node = *nfa.finish.borrow_mut();
        match mutable_node {
            Node::Epsilon { next, weak} => {
                *next = Some(new_final.clone());
                *weak = Some(nfa.start.clone());    
            },
            _ => {}
        }

        let new_start = Rc::new(RefCell::new(Node::DoubleEpsilon { 
            next: Some(nfa.start), 
            next2: Some(new_final.clone())
        }));

        SafeNfa { 
            start: new_start, 
            finish: new_final 
        }        
    }

    fn kleene_plus(nfa: SafeNfa) -> SafeNfa {
        todo!()
    }

    fn kleene_question(nfa: SafeNfa) -> SafeNfa {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use std::{println, rc::Rc, assert_eq, cell::RefCell};

    use crate::parser::NfaBuilder;

    use super::{SafeNfa, break_cycles};


    #[test]
    fn test2() {
        let first = SafeNfa::with_character_transition('a');
        let second = NfaBuilder::kleene_star(first);
        let third = NfaBuilder::kleene_star(SafeNfa::with_character_transition('b'));
        let fourth = NfaBuilder::concatenation(second, third);
        break_cycles(fourth.start);

        // let c = SafeNfa::with_epsilon_transition();
        // let second = SafeNfa::with_character_transition('b');
        // let second = SafeNfa::union(first, second);
        // drop(second.finish);
        // drop(first.start);
        // println!("{:?}", second);

        
    }
}

