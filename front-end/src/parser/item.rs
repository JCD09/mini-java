use crate::private::{symbol::Symbol, rule::Rule};

use super::grammar::RuleId;


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
pub(super) struct Item<'a, 'b>
where 'a: 'b 
{
    rule: &'b Rule<'a>,
    rule_id: RuleId,
    placeholder: usize,
    lookahead: Symbol<'a>
}

impl<'a, 'b> Item<'a, 'b>
where 'a: 'b 
{
    /// returns next symbol after placeholder
    fn new(rule: &'a Rule<'a>, rule_id: usize, lookahead: Symbol<'a>) -> Item<'a, 'b> {
        Item {
            rule,
            rule_id: rule_id.into(),
            placeholder: 0,
            lookahead
        }
    } 

    fn nontermial_at_placeholder(&self) -> Option<Symbol> {
        self.rule
            .body()
            .get(self.placeholder)
            .and_then(|symbol| {
                if symbol.is_nonterminal() {
                    Some(*symbol)
                } else {
                    None
                }
        })
    }

    pub(super) fn symbol_at_placeholder(&self) -> Option<Symbol> {
        self.rule.body()
            .get(self.placeholder)
            .copied()
    }

    pub(super) fn lookahead(&self) -> Symbol {
        self.lookahead
    }

    pub(super) fn advance_placeholder(&self) -> Option<Item<'a, 'b>> {
        if self.placeholder < self.rule.body().len() {
            Some(Item {
                rule: self.rule,
                rule_id: self.rule_id,
                placeholder: self.placeholder + 1,
                lookahead: self.lookahead
            })
        } else {
            None
        }
    }

    pub(super) fn rule_id(&self) -> RuleId {
        self.rule_id
    }
}

