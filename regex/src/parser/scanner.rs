use phf::{phf_map, phf_set, Set};

const OP_PAREN: char = '(';
const CL_PAREN: char = ')';
// const OP_BRACK: char = '[';
// const CL_BRACK: char = ']';

// const CARET: char = '^';
const BACKSLASH: char = '\\';

const ALTERNATION: char = '|';
const ASTERISK: char = '*';
const PLUS: char = '+';
const QUESTION_MARK: char = '?';

// const DOLLAR: char = '$';

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum Lexeme {
    Literal(char),

    OpParen, 
    ClParen, 

    Quantifier(char),
    Alternation(char),
    Concatenation(char),

    Escape(char),

    Epsilon,

    EoS,
    Error
}
pub(super) struct Scanner {
    tokens: Vec<Lexeme>
}

impl Scanner {
    pub(super) fn new(pattern: &str) -> Scanner {
        let mut lexemes = pattern.chars().map(|c| c.into()).collect::<Vec<Lexeme>>();
        lexemes.reverse();
        Scanner {
            tokens: lexemes
        }
    }

    pub(super) fn next(&mut self) -> Lexeme {
        self.tokens.pop().unwrap_or(Lexeme::EoS)
    }

    pub(super) fn peek(&self) -> Lexeme {
        self.tokens.last().copied().unwrap_or(Lexeme::EoS)
    }
}

impl From<char> for Lexeme {
    fn from(value: char) -> Self {
        match value {
            OP_PAREN => Lexeme::OpParen,
            CL_PAREN => Lexeme::ClParen,
            BACKSLASH => Lexeme::Escape('\\'),
            ALTERNATION => Lexeme::Alternation('|'),
            char @(ASTERISK|QUESTION_MARK|PLUS) => Lexeme::Quantifier(char),
            char@_ => Lexeme::Literal(char)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{assert_eq, println};


    
}