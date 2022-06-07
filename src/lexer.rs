#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Atom(char),
    Op(char),
    Eof,
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut tokens: Vec<Token> = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect();

        tokens.reverse();
        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn main() {}
