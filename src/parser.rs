use super::lexer::{Lexer, Token};
use super::s_expression::S;

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }
        lexer.next();

        let rhs = expr_bp(lexer, r_bp);

        lhs = S::Cons(op, vec![lhs, rhs]);
    }

    lhs
}

fn infix_binding_power(op: char) -> (u8, u8) {
    match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        '.' => (6, 5),
        _ => panic!("bad op: {:?}", op),
    }
}

#[test]
fn test_single_char() {
    let s = expr("1");
    assert_eq!(s.to_string(), "1");
}

#[test]
fn test_simple_addition() {
    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
}

#[test]
fn test_addition_and_multiplication() {
    let s = expr("a + b * c * d + e");
    assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");
}

#[test]
fn test_composition_operator() {
    let s = expr("f . g . h");
    assert_eq!(s.to_string(), "(. f (. g h))");
}

#[test]
fn test_misc_operators() {
    let s = expr("1 + 2 + f . g . h * 3 * 4");
    assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");
}
