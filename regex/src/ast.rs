use std::collections::VecDeque;

use crate::thompson::NFA;

const CAPACITY: usize = 1000;

#[derive(Clone)]
pub enum Expr {
    Empty, 
    Literal(Box<Literal>),
    Repetition(Box<KleeneStar>),
    Alternation(Box<Alternation>),
    Concatenation(Box<Concatenation>),
    Group(Box<Group>),
    Escape(Box<Escape>),
    MetaChar(Box<MetaChar>)
}

impl Expr {

}

#[derive(Clone)]
pub struct Literal {
    literal: char
}

impl Literal {
    pub fn new(literal: char) -> Literal {
        Literal { 
            literal 
        }
    }
}
#[derive(Clone)]
pub struct KleeneStar {
    expr: Expr
}

impl KleeneStar {
    pub fn new(expr: Expr) -> KleeneStar {
        KleeneStar { 
            expr    
        }
    }
}

#[derive(Clone)]
pub struct Alternation {
    left_expr: Expr,
    right_expr: Expr
}

impl Alternation {
    pub fn new(left: Expr, right: Expr) -> Alternation {
        Alternation { 
            left_expr: left, 
            right_expr: right
        }
    }
}

#[derive(Clone)]
pub struct Concatenation {
    left_expr: Expr,
    right_expr: Expr
}

impl Concatenation {
    pub fn new(left: Expr, right: Expr) -> Concatenation {
        Concatenation { 
            left_expr: left, 
            right_expr: right
        }
    }
}


#[derive(Clone)]
pub struct Group {
    expr: Expr
}

impl Group {
    pub fn new(expr: Expr) -> Group {
        Group {
            expr
        }
    }
}

#[derive(Clone)]
pub struct Escape {
    expr: Expr
}

impl Escape {
    pub fn new(expr: Expr) -> Escape {
        Escape { 
            expr    
        }
    }
}
pub trait Visitor {
    fn start(&self);
    fn visit_pre(&mut self, ast: &Expr);
    fn visit_in(&mut self, ast: &Expr);
    fn visit_post(&mut self, ast: &Expr);
    fn finish(&self, ast: &Expr);
}
type Nfa = (usize, usize);

pub struct ThompsonVisitor {
    args: VecDeque<Nfa>,
    nfa: NFA,
}

impl Visitor for ThompsonVisitor {
    fn start(&self) {    
    }
    fn visit_pre(&mut self, ast: &Expr) {
    }
    fn visit_in(&mut self, ast: &Expr) {
    }
    fn visit_post(&mut self, ast: &Expr) {  
    }
    fn finish(&self, ast: &Expr) {
    }
}

impl ThompsonVisitor {
    pub fn new() -> ThompsonVisitor {
      ThompsonVisitor {
        args: VecDeque::with_capacity(2),
        nfa: NFA::new()
      }  
    }

    // iterative (pre, post and in order) traversal of a tree using visiter pattern;
    pub fn visit(&mut self, ast: &Expr) {
        self.start();
        let mut active = VecDeque::<&Expr>::with_capacity(CAPACITY);
        let mut frames = VecDeque::<(&Expr, Option<&Expr>)>::with_capacity(CAPACITY);
        active.push_back(ast);
        loop {
            while let Some(ast_node) = active.pop_back() {
                self.visit_pre(ast_node);
                match ast_node {
                    Expr::Alternation(alt) => {
                        active.push_back(&alt.left_expr);
                        frames.push_back((ast_node, Some(&alt.right_expr)));
                    },
                    Expr::Concatenation(concat) => {
                        active.push_back(&concat.left_expr);
                        frames.push_back((ast_node, Some(&concat.right_expr)));
                    },
                    Expr::Repetition(rep) => {
                        active.push_back(&rep.expr);
                        frames.push_back((ast_node, None));
                    },
                    Expr::Group(group) => {
                        active.push_back(&group.expr);
                        frames.push_back((ast_node, None));
                    },
                    _ => {
                        frames.push_back((ast_node, None))
                    }
                }
            };
            

            while let Some(frame) = frames.pop_back() {
                match frame {
                    (node@Expr::Alternation(_), Some(next)) => {
                        self.visit_in(node);
                        frames.push_back((node, None));
                        active.push_back(&next);

                        break;
                    },
                    (node@Expr::Concatenation(_), Some(next)) => {
                        self.visit_in(node);
                        frames.push_back((node, None));
                        active.push_back(&next);

                        break;
                    }
                    _ => {
                        self.visit_post(frame.0);
                    }
                }
            }

            if active.is_empty() && frames.is_empty() {
                self.finish(ast);
                break;
            }
        }    
    }

}

// frame consists of a current ast node and next node that needs to be traversed by ast. 
struct Frame<'a> {
    // reference to current ast
    ast_node: &'a Expr,

    // and next link to traverse in the frame
    next: Option<&'a Expr>
}

impl Expr {
    
}


#[cfg(test)]
mod tests {
    use crate::{parser::parse, thompson};

    use super::ThompsonVisitor;


    #[test]
    fn wlak_expr_tree() {
        let pattern = "a|b|c";
        let expr = parse(pattern);
        let mut thompson = ThompsonVisitor::new();
        thompson.visit(&expr);

    }
}