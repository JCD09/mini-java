use core::panic;
use std::todo;

use phf::{phf_map, phf_set, Set};

const OP_PAREN: char = '(';
const CL_PAREN: char = ')';
const OP_BRACK: char = '[';
const CL_BRACK: char = ']';

const CARET: char = '^';
const BACKSLASH: char = '\\';

const ALTERNATION: char = '|';

const ASTERISK: char = '*';
const PLUS: char = '+';
const QUESTION_MARK: char = '?';

const DOLLAR: char = '$';

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum Token {
    Character(char),

    OpParen, 
    ClParen, 

    Operator(Type),

    Epsilon,

    EoS,
    Error
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Type {
    Prefix(char),
    Infix(char),
    Postfix(char)
}

impl Type {
    pub(super) fn symbol(&self) -> char {
        match self {
            Type::Prefix(op@_) => *op,
            Type::Infix(op@_) => *op,
            Type::Postfix(op@_) => *op,
        }
    }
}

pub(super) struct Scanner {
    tokens: Vec<Token>
}

impl Scanner {
    pub(super) fn new(pattern: &str) -> Scanner {
        let mut tokens = pattern.chars().map(|c| c.into()).collect::<Vec<Token>>();
        tokens.reverse();
        Scanner {
            tokens
        }
    }

    pub(super) fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::EoS)
    }

    pub(super) fn peek(&self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::EoS)
    }
}

// converts a series of characters into corresponding set of tokens
// inserts concatenation operator where it is appropriate

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            OP_PAREN => Token::OpParen,
            CL_PAREN => Token::ClParen,
            ALTERNATION => Token::Operator(Type::Infix(ALTERNATION)),
            char @(ASTERISK|QUESTION_MARK|PLUS) => Token::Operator(Type::Postfix(char)),
            char@_ => Token::Character(char)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{assert_eq, println};


    
}