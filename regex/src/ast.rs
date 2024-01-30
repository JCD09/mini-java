use std::collections::VecDeque;

use crate::thompson::ThompsonNfa;


const CAPACITY: usize = 1000;

#[derive(Clone)]
pub enum Expr {
    Empty, 
    Literal(Box<Literal>),
    Repetition(Box<KleeneStar>),
    Alternation(Box<Alternation>),
    Concatenation(Box<Concatenation>),
    Group(Box<Group>)
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

pub trait Visitor {
    fn start(&self);
    fn visit_pre(&mut self, ast: &Expr);
    fn visit_in(&mut self, ast: &Expr);
    fn visit_post(&mut self, ast: &Expr);
    fn finish(&self, ast: &Expr);
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


pub struct ThompsonVisitor {
    nfa: ThompsonNfa,
}

impl ThompsonVisitor {
    pub fn new() -> ThompsonVisitor {
      ThompsonVisitor {
        nfa: ThompsonNfa::new()
      }  
    }

    // iterative (and in order) DFS traversal of a tree
    pub fn visit(&mut self, ast: &Expr) {
        self.start();
        let mut active = VecDeque::<&Expr>::with_capacity(CAPACITY);
        let mut frames = VecDeque::<Frame>::with_capacity(CAPACITY);
        active.push_back(ast);
        loop {
            while let Some(ast_node) = active.pop_back() {
                self.visit_pre(ast_node);
                match ast_node {
                    Expr::Alternation(alt) => {
                        active.push_back(&alt.left_expr);
                        frames.push_back(Frame { ast_node, next: Some(&alt.right_expr) });
                    },
                    Expr::Concatenation(concat) => {
                        active.push_back(&concat.left_expr);
                        frames.push_back(Frame { ast_node, next: Some(&concat.right_expr) });
                    },
                    Expr::Repetition(rep) => {
                        active.push_back(&rep.expr);
                        frames.push_back(Frame { ast_node, next: None });
                    },
                    Expr::Group(group) => {
                        active.push_back(&group.expr);
                        frames.push_back(Frame { ast_node, next: None });
                    },
                    _ => {
                        frames.push_back(Frame { ast_node, next: None })
                    }
                }
            };
            

            while let Some(frame) = frames.pop_back() {
                match frame {
                    Frame { ast_node: Expr::Alternation(_), next: Some(next_ast)} => {
                        self.visit_in(frame.ast_node);
                        frames.push_back(Frame { ast_node: frame.ast_node, next: None });
                        active.push_back(next_ast);
                        break;
                    },
                    Frame { ast_node: Expr::Concatenation(rep), next: Some(next_ast)} => {
                        self.visit_in(frame.ast_node);
                        frames.push_back(Frame { ast_node: frame.ast_node, next: None });
                        active.push_back(next_ast);
                        break;
                    },
                    _ => {
                        self.visit_post(frame.ast_node);
                    }
                }
            }

            if active.is_empty() && frames.is_empty() {
                break;
            }
        }    
        self.finish(ast);
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