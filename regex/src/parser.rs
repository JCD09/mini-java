use std::{str::Chars, todo};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RegexToken {
    Start,
    Character(char),
    CharacterClass,
    Operator(char),
    End,
    Error
}

// Converts a string of characters into a vector of tokens and in the process inserts concatenation 
// operator where appropriate; 
pub fn into_tokens<'a>(chars: &'a str) -> Vec<RegexToken> {
    let capacity = chars.len();
    let mut regex_tokens = Vec::<RegexToken>::with_capacity(capacity);
    regex_tokens.push(RegexToken::Start);
    let mut it = chars.chars().peekable();
    while let Some(regex_symbol) = it.next() {
        match regex_symbol {
            '\\' => match (it.next(), it.peek()) {
                (
                    Some(c@('('|')'|'*'|'+'|'?'|'|'|'\\')), 
                    None
                ) => {
                    regex_tokens.push(RegexToken::Character(c)) 
                },
                (
                    Some(c@('('|')'|'*'|'+'|'?'|'|')), 
                    Some(')'|'*'|'+'|'?'|'|')
                ) => {
                    regex_tokens.push(RegexToken::Character(c))
                },
                (Some(c@('('|')'|'*'|'+'|'?'|'|')) , _) => {
                    regex_tokens.push(RegexToken::Character(c));
                    regex_tokens.push(RegexToken::Operator('.'));
                },
                _ => regex_tokens.push(RegexToken::Error)
            }
            op@('('|')'|'*'|'+'|'?'|'|')=> {
                regex_tokens.push(RegexToken::Operator(op))
            }
            c@_ => match it.peek() {
                None | Some(')') | Some('*') |Some('+') | Some('?') | Some('|') => {
                    regex_tokens.push(RegexToken::Character(c))
                },
                _ => {
                    regex_tokens.push(RegexToken::Character(c));
                    regex_tokens.push(RegexToken::Operator('.'));
                }
            }
        }
    }
    regex_tokens.push(RegexToken::End);
    regex_tokens
}

#[cfg(test)]
mod tests {

    use std::assert_eq;

    use crate::parser::RegexToken;

    use super::{into_tokens};

    #[test]
    fn empty_string() {
        let empty_string = "";
        let tokens = into_tokens(empty_string);
        let result = vec![RegexToken::Start, RegexToken::End];
        assert_eq!(tokens,result);
    }

    #[test]
    fn single_char() {
        let single_char = "a";
        let tokens = into_tokens(single_char);
        let result = vec![
            RegexToken::Start, 
            RegexToken::Character('a'),
            RegexToken::End
        ];
        assert_eq!(tokens,result);
    }

    #[test]
    fn backslash_char() {
        let backslash_char = "\\(";
        let tokens = into_tokens(backslash_char);
        let result = vec![
            RegexToken::Start, 
            RegexToken::Character('('),
            RegexToken::End
        ];
        assert_eq!(tokens,result);
    }

    #[test]
    fn concat_two_chars() {
        let two_char = "ab";
        let tokens = into_tokens(two_char);
        let result = vec![
            RegexToken::Start, 
            RegexToken::Character('a'),
            RegexToken::Operator('.'),
            RegexToken::Character('b'),
            RegexToken::End
        ];
        assert_eq!(tokens,result);
    }

    #[test]
    fn alternation() {
        let alternation = "a|b";
        let tokens = into_tokens(alternation);
        let result = vec![
            RegexToken::Start, 
            RegexToken::Character('a'),
            RegexToken::Operator('|'),
            RegexToken::Character('b'),
            RegexToken::End
        ];
        assert_eq!(tokens,result);
    }

    #[test]
    fn with_backslash() {
        let alternation = "a|\\\\";
        let tokens = into_tokens(alternation);
        let result = vec![
            RegexToken::Start, 
            RegexToken::Character('a'),
            RegexToken::Operator('|'),
            RegexToken::Character('\\'),
            RegexToken::End
        ];
        assert_eq!(tokens,result);
    }

    #[test]
    fn character_with_grouping() {
        let alternation = "a(b|c)";
        let tokens = into_tokens(alternation);
        let result = vec![
            RegexToken::Start, 
            RegexToken::Character('a'),
            RegexToken::Operator('.'),
            RegexToken::Operator('('),
            RegexToken::Character('b'),
            RegexToken::Operator('|'),
            RegexToken::Character('c'),
            RegexToken::Operator(')'),
            RegexToken::End
        ];
        assert_eq!(tokens,result);
    }


    
}