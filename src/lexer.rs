use token;
use token::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {input: input.chars().peekable()}
    }

    pub fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&c) => is_letter(c),
            None => false
        }
    }

    pub fn read_identifier(&mut self, first_char: char) -> String {
        let mut identifier = String::new();
        identifier.push(first_char);
        while self.peek_is_letter() {
            identifier.push(self.read_char().unwrap());
        }
        identifier
    }

    pub fn next_token(&mut self) -> Token {
        match self.read_char() {
            Some('/') => Token::Frac,
            Some(c @ _) => {
                if is_letter(c) {
                    let identifier = self.read_identifier(c);
                    token::indent_lookup(&identifier)
                } else {
                    Token::Illegal
                }
            }
            None => Token::EOF
            }
        }
    }


fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[test]
fn is_letter_test() {
    assert!(is_letter('_'));
    assert!(is_letter('a'));
    assert!(is_letter('Z'));

    assert!(!is_letter('*'));
    assert!(!is_letter('1'));
}
