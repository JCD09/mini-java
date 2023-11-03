use std::{todo, collections::{VecDeque, HashMap, HashSet}};
use phf::phf_map;
use std::alloc::{alloc,alloc_zeroed,dealloc,Layout};

use crate::parser::scanner::{Scanner};

mod scanner; 
mod set_nfa;
// mod safe_thomson_nfa;
// mod adjacency_thomson_nfa;

const CONCATENATION: char = 'c';

#[derive(Copy,Clone)]
struct Precedence((Option<usize>,Option<usize>)); 

impl Precedence {
    fn left_bp(&self) -> Option<&usize> {
        self.0.0.as_ref()
    }
    fn right_bp(&self) -> Option<&usize> {
        self.0.1.as_ref()
    }
}


pub trait Nfa {
    fn with_epsilon_transition() -> Self;
    fn with_character_transition(transition_symbol: char) -> Self;
    fn concatenation(left: Self, right: Self) -> Self; 
    fn union(left: Self, right: Self) -> Self; 
    fn kleene_star(nfa: Self) -> Self; 
    fn kleene_plus(nfa: Self) -> Self; 
    fn kleene_question(nfa: Self) -> Self; 
}

// Operators have precedence and associativity, both are encoded as left binding power and right binding power.
// To encode left associativity op1_lbp >= op2_rbp, assuming that op1 and op2 are the same operator, but left 
// associative. This way, operator2 will be placed on the output queue. 
// In the other way around, if op1_lbp < op2_rbp, when op2 will be placed onto the stack instead and operators will 
// right associative.  
static OPERATOR_PRECEDENCE: phf::Map<char, Precedence> = phf_map! {
    '(' => Precedence((Some(100), Some(100))),
    ')' => Precedence((Some(100), Some(100))),
    '*' => Precedence((Some(80),Some(80))),     // unary
    '?' => Precedence((Some(80),Some(80))),     // unary
    '+' => Precedence((Some(80),Some(80))),     // unary
    'c' => Precedence((Some(70),Some(70))), // binary, left associative,  
    '|' => Precedence((Some(60),Some(60))), // binary, left associative, 
};

const MAX_PRECEDENCE: Precedence = Precedence((Some(usize::MAX),Some(usize::MAX)));
const MIN_PRECEDENCE: Precedence = Precedence((Some(0),Some(0)));

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RegexToken {
    Character(char),
    Operator(char),
    Error
}

#[derive(Debug)]
enum NotDefined {
    Chracter(char),
    Concatenation,
    Union,
    Kleene,
    OpParen,
    ClParen,
}

impl From<char> for NotDefined {
    fn from(value: char) -> Self {
        match value {
            'c' => Self::Concatenation,
            '|' => Self::Union,
            '*' => Self::Kleene,
            '(' => Self::OpParen,
            ')' => Self::ClParen,
            c@_ => {
                unimplemented!()
            }
        }
    }
}


// converts given regular expression into corresponding postfix expression using 
// shunting yard algorithm. 
fn to_posfix(regex: &str) -> Vec<NotDefined> {
    let capacity = regex.len();

    let mut input = Scanner::new(regex).iter().peekable();
    let mut operators = Stack::with_capacity(capacity);
    let mut output = Vec::<NotDefined>::with_capacity(capacity);
    while let Some(regex_token) = input.peek() {
        match regex_token {
            regex_char@RegexToken::Character(c@_) => {
                output.push(NotDefined::Chracter(*c));
                input.next();
                match input.peek() {
                    Some(RegexToken::Character(c@_)) => {
                        operators.iter(CONCATENATION)
                            .for_each(|el|output.push(el.into()));
                        operators.push(CONCATENATION);
                    },
                    Some(RegexToken::Operator('(')) => {
                        operators.iter(CONCATENATION) // needs to be adjusted 
                            .for_each(|el|output.push(el.into()));
                        operators.push(CONCATENATION);
                    }
                    _ => {}
                }
            },
            RegexToken::Operator(opr@')') => {
                operators.iter(')')
                    .for_each(|el| output.push(el.into()));
                operators.pop();
                input.next();        
            },
            RegexToken::Operator(opr@_) => {
                match operators.peek() {
                    Some(top@_) => {
                        operators.iter(top)
                            .for_each(|el| output.push(el.into()));
                        operators.push(*opr);
                        input.next();
                    }, 
                    None => {
                        operators.push(*opr);
                        input.next();
                    },
                }
            }
            RegexToken::Error => todo!(),
        }
    }
    operators.inner.drain(..).for_each(|el|output.push(el.into()));
    output
}

// this function evaluates 
fn evaluate<T: Nfa, N: Nfa>(tokens: &[RegexToken], nfa_builder: T) -> N {
    let mut result = VecDeque::<N>::with_capacity(tokens.len());
    tokens.iter()
        .for_each(|token| {
           match token {
            RegexToken::Character(c@_) => {
                let b = N::with_character_transition(*c);
                result.push_back(b);  
            },
            RegexToken::Operator('c') => {
                match (result.pop_back(), result.pop_back()) {
                    (None, None) => todo!(),
                    (None, Some(_)) => todo!(),
                    (Some(_), None) => todo!(),
                    (Some(first@_), Some(second@_)) => {
                        result.push_back(N::concatenation(first, second));
                    },
                }
            },
            RegexToken::Operator('|') => {
                match (result.pop_back(), result.pop_back()) {
                    (None, None) => todo!(),
                    (None, Some(_)) => todo!(),
                    (Some(_), None) => todo!(),
                    (Some(first@_), Some(second@_)) => {
                        result.push_back(N::union(first, second));
                    },
                }
            },
            RegexToken::Operator('*') => {
                match result.pop_back() {
                    Some(first@_) => {
                        result.push_back(N::kleene_star(first));
                    },
                    None => todo!(),
                }
            },
            RegexToken::Operator(_) => todo!(),
            RegexToken::Error => todo!(),
        }
        });
    todo!()
}





pub struct Stack {
    inner: VecDeque<char>
}

impl Stack {
    fn with_capacity(capacity: usize) -> Self {
        Stack {
            inner: VecDeque::with_capacity(capacity)
        }
    }
    fn push(&mut self, item: char) {
        self.inner.push_back(item);
    }
    fn pop(&mut self) -> Option<char> {
        self.inner.pop_back()
    }
    fn peek(&self) -> Option<char> {
        self.inner.back().copied()
    }

    // returns iterator over stack elements until the bottom operator is null or the the bottom 
    // operator has precedence that is higher than the provided one. 
    // for example if the stack has operators with precedence 70/61/50/40 and the next 
    // operator has the precedence 60, then mutable iterator over 40,50 will be returned. 

    fn iter(&mut self, operator: char) -> Operators {
        let it = VecDeque::<char>::with_capacity(self.inner.len());

        let precedence = *OPERATOR_PRECEDENCE.get(&operator)
            .unwrap_or(&MIN_PRECEDENCE); 

        Operators { 
            container: &mut self.inner, 
            next_op_precedence: precedence 
        }
    }
}

pub struct Operators<'a> {
    container: &'a mut VecDeque<char>,
    next_op_precedence: Precedence
}

impl<'a> Iterator for Operators<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char@_) = self.container.back() {

            let stack_op_precedence = OPERATOR_PRECEDENCE.get(char)
                .unwrap_or(&MIN_PRECEDENCE);

            if stack_op_precedence.left_bp() > self.next_op_precedence.right_bp() {
                return self.container.pop_back();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::to_posfix;


    #[test]
    fn test() {

        let regex = "(e*|f)";
        let posfix = to_posfix(regex);
        println!("result {:?}", &posfix);

    }
}
