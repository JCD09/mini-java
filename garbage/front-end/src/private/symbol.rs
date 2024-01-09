// inner facing api
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub(crate) enum Symbol<'a> {
    Goal,
    NonTerminal(&'a str),
    Keyword(&'a str),
    Element(&'a str),
    EmptyString,
    EndOfFile,
}

impl<'a> Symbol<'a> {
    pub(crate) fn is_nonterminal(&self) -> bool {
        match self {
            Symbol::NonTerminal(_) => true,
            &Symbol::Goal => true,
            _ => false
        }
    }
}


