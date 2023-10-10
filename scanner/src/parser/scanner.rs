use std::{str::Chars, todo};

#[derive(Debug,Clone, Copy)]
pub(super) enum Token {
    Character(char),
    Operator(char),
    Error
}

pub(super) struct Scanner<'a> {
    regex: &'a str
}

impl<'a> Scanner<'a> {
    pub(super) fn new(regex: &'a str) -> Scanner {
        Scanner {
            regex
        }
    }

    pub(super) fn iter(&self) -> RegexIter<'a> {
        RegexIter { 
            it: self.regex.chars()
        }
    }
}

pub(super) struct RegexIter<'a> {
    it: Chars<'a>
}

impl<'a> RegexIter<'a> {

    pub(super) fn backslash(&mut self, symbol: char) -> Token {
        if let Some(symbol) = self.it.next() {
            return match symbol {
                '(' => Token::Character('('), 
                ')' => Token::Character(')'), 
                '*' => Token::Character('*'), 
                '+' => Token::Character('+'),
                '?' => Token::Character('?'),
                '|' => Token::Character('|'), 
                '[' => Token::Character('['),
                ']' => Token::Character(']'),
                _ => Token::Error
            }
        }
        Token::Error
    }       
}

impl<'a> Iterator for RegexIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.it.next() {
            match symbol {
                '(' => Some(Token::Operator('(')),
                ')' => Some(Token::Operator(')')), 
                '*' => Some(Token::Operator('*')), 
                '+' => Some(Token::Operator('+')),
                '?' => Some(Token::Operator('?')),
                '|' => Some(Token::Operator('|')), 
                // '[' => Some(RegexToken::Operator('[')),
                // ']' => Some(RegexToken::Operator(']')),
                '\\' => Some(self.backslash(symbol)),

                _ => Some(Token::Character(symbol))

            };
        }
        None
    }
}