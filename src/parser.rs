use super::lexer::{Lexer, Token};
use super::s_expression::S;

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer)
}

fn expr_bp(lexer: &mut Lexer) -> S {
    let lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.next() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        todo!();
    }

    lhs
}

#[test]
fn tests() {
    let s = expr("1 + 2 + 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
}

#[test]
fn test_single_char() {
    let s = expr("1");
    assert_eq!(s.to_string(), "1");
}
