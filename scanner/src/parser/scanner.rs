use std::{str::Chars, todo};

use super::RegexToken;


#[derive(Debug)]
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

#[derive(Debug)]
pub(super) struct RegexIter<'a> {
    it: Chars<'a>
}

impl<'a> RegexIter<'a> {

    pub(super) fn backslash(&mut self, symbol: char) -> RegexToken {
        if let Some(symbol) = self.it.next() {
            return match symbol {
                '(' => RegexToken::Character('('), 
                ')' => RegexToken::Character(')'), 
                '*' => RegexToken::Character('*'), 
                '+' => RegexToken::Character('+'),
                '?' => RegexToken::Character('?'),
                '|' => RegexToken::Character('|'), 
                '[' => RegexToken::Character('['),
                ']' => RegexToken::Character(']'),
                _ => RegexToken::Error
            }
        }
        RegexToken::Error
    }       
}

impl<'a> Iterator for RegexIter<'a> {
    type Item = RegexToken;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.it.next() {
            return match symbol {
                '(' => Some(RegexToken::Operator('(')),
                ')' => Some(RegexToken::Operator(')')), 
                '*' => Some(RegexToken::Operator('*')), 
                '+' => Some(RegexToken::Operator('+')),
                '?' => Some(RegexToken::Operator('?')),
                '|' => Some(RegexToken::Operator('|')), 
                // '[' => Some(RegexToken::Operator('[')),
                // ']' => Some(RegexToken::Operator(']')),
                '\\' => Some(self.backslash(symbol)),

                _ => Some(RegexToken::Character(symbol))

            };
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::scanner::RegexToken;

    use super::Scanner;

    #[test]
    fn test() {
        let regex = "abcd";
        let scaner = Scanner::new(regex);
        let result = Vec::from([RegexToken::Character('a'),RegexToken::Character('b'),RegexToken::Character('c'),RegexToken::Character('d')]);
        let output = scaner.iter().collect::<Vec<RegexToken>>();

        assert_eq!(result,output);
    }
}