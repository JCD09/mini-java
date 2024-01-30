
use std::iter::Peekable;

use crate::ast::{Expr, Group, Concatenation, Alternation, KleeneStar, Literal};

const MIN_PRECEDENCE: usize = 0;
const ALTERNATION_LBP: usize = 25;
const ALTERNATION_RBP: usize = 30; 
const CONCATENATION_LBP: usize = 35; 
const CONCATENATION_RBP: usize = 40; 
const STAR_LBP: usize = 50;


const ESCAPE: char = '\\';

const META: [char;15] = ['.','^','$','*','+','?','{','}','[',']','\\','|','(',')',','];
const CC_META: [char;4] = ['-','^','\\',']'];

const QUANTIFIER: [char;3] = ['*','+','?'];

pub fn parse(pattern: &str) -> Expr {
    let mut scanner = pattern.chars().peekable();

    if let None = scanner.peek() {
        return Expr::Empty;
    }

    parse_regex(&mut scanner, MIN_PRECEDENCE)
}

#[derive(Debug,Eq,PartialEq)]
enum Operator {
    Alternation(usize,usize),
    Concatenation(usize,usize),
    KleeneStar(usize),
    ZeroOrOne(usize),
    OneOrMany(usize)
    // Star(usize),
    // Question(usize),
    // Plus(usize),
    
}

impl Operator {
    fn alternation() -> Operator {
        Operator::Alternation(ALTERNATION_LBP, ALTERNATION_RBP)
    }
    fn concatenation() -> Operator {
        Operator::Concatenation(ALTERNATION_LBP,ALTERNATION_RBP)
    }
    fn kleene_star() -> Operator {
        Operator::KleeneStar(STAR_LBP)
    }
    fn zero_or_one() -> Operator {
        Operator::ZeroOrOne(STAR_LBP)
    }
    fn one_or_many() -> Operator {
        Operator::OneOrMany(STAR_LBP)
    }
}

fn parse_regex(scanner: &mut Peekable<impl Iterator<Item = char>>, min_precedence: usize) -> Expr {
    let mut expr = match scanner.next() {
        Some('(') => {
            let expr = parse_regex(scanner, MIN_PRECEDENCE);
            let Some(')') = scanner.next() else {
                panic!("no matching \")\" found");
            };
            
            Expr::Group(Box::new(Group::new(expr)))
        },
        Some(char@_) if !META.contains(&char) => {
            let literal = Literal::new(char);
            Expr::Literal(Box::new(literal))
        },
        _ => {
            panic!("invalid pattern")
        }
    };

    loop {
        let operator = match scanner.peek() {
            Some(char@_) if !META.contains(&char) => Operator::concatenation(),
            Some('(') => Operator::concatenation(),
            Some('|') => Operator::alternation(),
            Some('*') => Operator::kleene_star(),
            Some('?') => Operator::zero_or_one(),
            Some('+') => Operator::one_or_many(),
            Some(')') => break,
            None => break,
            _ => {
                panic!("invalid operator")
            }
        };

        match operator {
            Operator::Alternation(left_bp@_,right_bp) => {
                if left_bp < min_precedence {
                    break;
                }
                scanner.next();
                let right_expr = parse_regex(scanner, right_bp);
                let alternation = Alternation::new(expr, right_expr);
                expr = Expr::Alternation(Box::new(alternation));

                continue;
            },
            Operator::Concatenation(left_bp@_,right_bp@_) => {
                if left_bp < min_precedence {
                    break;
                }
                let right_expr = parse_regex(scanner, right_bp);
                let concat = Concatenation::new(expr, right_expr);
                expr = Expr::Concatenation(Box::new(concat));

                continue;
            },
            Operator::KleeneStar(left) => {
                let star = KleeneStar::new(expr);
                expr = Expr::Repetition(Box::new(star));

                continue;
            },
            Operator::ZeroOrOne(left) => {
                let empty = Expr::Empty;
                let alternation = Alternation::new(expr,empty);
                let inner_expr = Expr::Alternation(Box::new(alternation));
                let group = Group::new(inner_expr);
                expr = Expr::Group(Box::new(group));

                continue;
            },
            Operator::OneOrMany(left) => {
                let clone = expr.clone();
                let star = KleeneStar::new(clone);
                let right_expr = Expr::Repetition(Box::new(star));
                let concat = Concatenation::new(expr, right_expr);
                let inner = Expr::Concatenation(Box::new(concat));
                let group = Group::new(inner);
                expr = Expr::Group(Box::new(group));

                continue;
            },
        };
    }
    expr
}

fn parse_character_class(scanner: &mut impl Iterator<Item=char>) -> Expr {
    todo!()
}

fn parse_quantifier(scanner: &mut impl Iterator<Item=char>) -> Expr {
    todo!()
}
