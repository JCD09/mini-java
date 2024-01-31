
use std::iter::Peekable;

use crate::ast::{Expr, Group, Concatenation, Alternation, KleeneStar, Literal, Escape};

const MIN_PRECEDENCE: usize = 0;

const ALTERNATION_LBP: usize = 25;
const ALTERNATION_RBP: usize = 30; 

const CONCATENATION_LBP: usize = 35; 
const CONCATENATION_RBP: usize = 40; 

const STAR_LBP: usize = 50;

const OP_PAREN_RBP: usize = 0;
const CL_PAREN_RBP: usize = 70;

const ESCAPE_RBP: usize = 80;

const OP_PAREN: char = '(';
const CL_PAREN: char = ')';

const PIPE: char = '|';

const STAR: char = '*';
const QUESTION: char = '?';
const PLUS: char = '+';

const ESCAPE: char = '\\';

const META: [char;15] = ['.','^','$','*','+','?','{','}','[',']','\\','|','(',')',','];
const CC_META: [char;4] = ['-','^','\\',']'];

const QUANTIFIER: [char;3] = ['*','+','?'];

pub fn parse(pattern: &str) -> Expr {
    let mut scanner = pattern.chars().peekable();

    if let None = scanner.peek() {
        return Expr::Empty;
    }

    parse_reg_expr(&mut scanner, MIN_PRECEDENCE)
}

// Nud and Led are terms from paper by Pratt trhat denote a token that can start an expression
// usually 

// Nud tokens are tokens that start exression
enum Nud {
    Char(char),
    Escape(usize),
    OpParen(usize),
}

impl Nud {
    fn character(char: char) -> Nud {
        Nud::Char(char)
    }
    fn escape() -> Nud {
        Nud::Escape(ESCAPE_RBP)
    }
    fn op_paren() -> Nud {
        Nud::OpParen(MIN_PRECEDENCE)
    }
}

fn nud_token(char: char) -> Nud {
    match char {
        OP_PAREN => Nud::op_paren(),
        ESCAPE => Nud::escape(),
        char if !META.contains(&char) => Nud::Char(char),
        _ => panic!("invalid Nud token symbol")
    }
}

#[derive(Debug,Eq,PartialEq)]
enum Led {
    Alternation(usize,usize),
    Concatenation(usize,usize),
    Repetition(usize),

    Break
}


impl Led {
    fn alternation() -> Led {
        Led::Alternation(ALTERNATION_LBP, ALTERNATION_RBP)
    }
    fn concatenation() -> Led {
        Led::Concatenation(ALTERNATION_LBP,ALTERNATION_RBP)
    }
    fn repetiton() -> Led {
        Led::Repetition(STAR_LBP)
    }
}

fn led_token(char: char) -> Led {
    match char {
        _ => todo!()
    }
}

fn parse_reg_expr(scanner: &mut Peekable<impl Iterator<Item = char>>, min_prec: usize) -> Expr {
    if let Some(char) = scanner.next() {
        // handle Nud case  
        let mut expr = match nud_token(char) {
            Nud::OpParen(min_rbp) => {
                let expr = parse_reg_expr(scanner, min_rbp);
                let Some(CL_PAREN) = scanner.next() else {
                    panic!("no matching \")\"")
                };
                Expr::Group(Box::new(Group::new(expr)))
            }
            // backslash is also a metacharacter so the next element must be meta_charater
            // after a few hours of fiddling around I, I decided that it is the 
            // best to handle this case inside Nud Operator. 
            // and then add other characters from the inside corresponding match match 
            Nud::Escape(min_rbp) => {
                match scanner.next() {
                    Some(c) if META.contains(&c) => {
                        Expr::Literal(Box::new(Literal::new(char)))
                    }
                    _ => panic!("invalid character")
                }
            },
            Nud::Char(char) => todo!()
        };

        // handle led case
        while let Some(lexeme) = scanner.peek() {
            let op = match led_token(*lexeme) {
                Led::Alternation(left_bp, right_bp) =>  {
                    if left_bp < min_prec {
                        break;
                    }

                    scanner.next();
                    let right_expr = parse_reg_expr(scanner, right_bp);
                    let alternation = Alternation::new(expr, right_expr);
                    expr = Expr::Alternation(Box::new(alternation));
                    
                    continue;
                },
                Led::Concatenation(left_bp, right_bp) => {
                    if left_bp < min_prec {   
                        break;
                    }
                    let right_expr = parse_reg_expr(scanner, right_bp);
                    let concat = Concatenation::new(expr, right_expr);
                    expr = Expr::Concatenation(Box::new(concat));
                    
                    continue;
                },
                Led::Repetition(left_bp) => {
                    let star = KleeneStar::new(expr);
                    expr = Expr::Repetition(Box::new(star));
                    
                    continue;
                },
                Led::Break => break,
            };
        }
        return expr;
    }
    Expr::Empty

    // loop {
    //     let operator = match scanner.peek() {
    //         Some(char@_) if !META.contains(&char) => Led::concatenation(),
    //         Some('(') => Led::concatenation(),
    //         Some('|') => Led::alternation(),
    //         Some('*') => Led::kleene_star(),
    //         Some('?') => Led::zero_or_one(),
    //         Some('+') => Led::one_or_many(),
    //         Some(')') => break,
    //         None => break,
    //         _ => {
    //             panic!("Encountered invalid operator")
    //         }
    //     };

    //     match operator {
    //         Led::Alternation(left_bp@_,right_bp) => {
    //             if left_bp < min_prec {
    //                 break;
    //             }
    //             scanner.next();
    //             let right_expr = parse_reg_expr(scanner, right_bp);
    //             let alternation = Alternation::new(expr, right_expr);
    //             expr = Expr::Alternation(Box::new(alternation));

    //             continue;
    //         },
    //         Led::Concatenation(left_bp@_,right_bp@_) => {
    //             if left_bp < min_prec {
    //                 break;
    //             }
    //             let right_expr = parse_reg_expr(scanner, right_bp);
    //             let concat = Concatenation::new(expr, right_expr);
    //             expr = Expr::Concatenation(Box::new(concat));

    //             continue;
    //         },
    //         Led::Repetition(left) => {
    //             let star = KleeneStar::new(expr);
    //             expr = Expr::Repetition(Box::new(star));

    //             continue;
    //         },
    //         Led::ZeroOrOne(left) => {
    //             let empty = Expr::Empty;
    //             let alternation = Alternation::new(expr,empty);
    //             let inner_expr = Expr::Alternation(Box::new(alternation));
    //             let group = Group::new(inner_expr);
    //             expr = Expr::Group(Box::new(group));

    //             continue;
    //         },
    //         Led::OneOrMany(left) => {
    //             let clone = expr.clone();
    //             let star = KleeneStar::new(clone);
    //             let right_expr = Expr::Repetition(Box::new(star));
    //             let concat = Concatenation::new(expr, right_expr);
    //             let inner = Expr::Concatenation(Box::new(concat));
    //             let group = Group::new(inner);
    //             expr = Expr::Group(Box::new(group));

    //             continue;
    //         },
    //     };
    // }

}

fn parse_character_class(scanner: &mut impl Iterator<Item=char>) -> Expr {
    todo!()
}

fn parse_quantifier(scanner: &mut impl Iterator<Item=char>) -> Expr {
    todo!()
}
