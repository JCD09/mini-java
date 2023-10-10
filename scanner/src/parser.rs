use std::{todo, collections::{VecDeque, HashMap, HashSet}};
use phf::phf_map;
use std::alloc::{alloc,alloc_zeroed,dealloc,Layout};

use crate::parser::scanner::{Scanner, Token};

mod scanner; 
mod set_nfa;
// mod safe_thomson_nfa;
// mod adjacency_thomson_nfa;

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
    fn one();
    fn two();
}

pub trait NfaBuilder {
    fn with_epsilon_transition() -> Self;
    fn with_character_transition(transition_symbol: char) -> Self;
    fn concatenation(left: Self, right: Self) -> Self; 
    fn union(left: Self, right: Self) -> Self; 
    fn kleene_star(nfa: Self) -> Self; 
    fn kleene_plus(nfa: Self) -> Self; 
    fn kleene_question(nfa: Self) -> Self; 
}

// Operators have precedence which is encoded as left binding power and right binding power. 
// If we want to encode left associativity then operator left bp must be smaller than right 
// binding power. this way when operators are compared, top operator from the stack will 
// have smaller precednece than the next operator stack_op.lbp < stack_op.rbp so stack op precedence 
// will be lower then next operator precedence and during the iterator
// it will be poped from the stack added to the output. 
static OPERATOR_PRECEDENCE: phf::Map<char, Precedence> = phf_map! {
    '(' => Precedence((None, Some(100))),
    ')' => Precedence((Some(101), None)),
    '*' => Precedence((Some(80),Some(80))),     // unary
    '?' => Precedence((Some(80),Some(80))),     // unary
    '+' => Precedence((Some(80),Some(80))),     // unary
    'c' => Precedence((Some(71),Some(70))), // binary, left associative,  
    '|' => Precedence((Some(61),Some(60))), // binary, left associative, 
};

const MAX_PRECEDENCE: Precedence = Precedence((Some(usize::MAX),Some(usize::MAX)));
const MIN_PRECEDENCE: Precedence = Precedence((Some(0),Some(0)));

// converts given regular expression into corresponding postfix expression using 
// shunting yard algorithm. 
fn to_posfix(regex: &str) -> Vec<Token> {
    let capacity = regex.len();

    let mut input = Scanner::new(regex).iter().peekable();
    let mut operators = Stack::with_capacity(capacity);
    let mut output = Vec::<Token>::with_capacity(capacity);

    while let Some(regex_token) = input.peek() {
        match regex_token {
            regex_char@Token::Character(_) => {
                output.push(*regex_char);
                input.next();
                match input.peek() {
                    Some(Token::Character(_)) => {
                        operators.iter('c')
                            .for_each(|el|output.push(Token::Operator(el)));
                        operators.push('c');
                    },
                    Some(Token::Operator('(')) => {
                        operators.iter('c')
                            .for_each(|el|output.push(Token::Operator(el)));
                        operators.push('c');
                    }
                    _ => {}
                }
            },
            Token::Operator(opr) => {
                if let Some(top) = operators.peek() {
                    operators.iter(top)
                        .for_each(|el| output.push(Token::Operator(el)));
                    operators.push(*opr);
                    input.next();
                } else {
                    operators.push(*opr);
                    input.next();
                }
            },
            Token::Error => todo!(),
        }
    }
    operators.inner.drain(..).for_each(|el|output.push(Token::Operator(el)));
    output
}

// this function evaluates 
fn evaluate<T: NfaBuilder, N: Nfa>(tokens: &[Token], nfa_builder: T) -> N {
    let stack = VecDeque::<N>::with_capacity(tokens.len());
    tokens.iter()
        .map(|token| {
           
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
            precedence 
        }
    }
}

pub struct Operators<'a> {
    container: &'a mut VecDeque<char>,
    precedence: Precedence
}

impl<'a> Iterator for Operators<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char@_) = self.container.back() {

            let precedence = OPERATOR_PRECEDENCE.get(char)
                .unwrap_or(&MAX_PRECEDENCE);

            if precedence.left_bp() < self.precedence.right_bp() {
                return self.container.pop_back();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::println;


    #[test]
    fn test() {

        let arr = [1,2,3,4,5];

    }
}
