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

    pub fn read_next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&c) => c.is_alphabetic(),
            None => false
        }
    }

    pub fn read_identifier(&mut self, first_char: char) -> String {
        let mut identifier = String::new();
        identifier.push(first_char);
        while self.peek_is_letter() {
            identifier.push(self.read_next_char().unwrap());
        }
        identifier
    }

    pub fn next_token(&mut self) -> Token {
        match self.read_next_char() {
            Some('/') => Token::Frac,
            Some(c) => {
                if c.is_alphabetic() {
                    let identifier = self.read_identifier(c);
                    token::indent_lookup(&identifier)
                } else if c.is_whitespace() {
                    Token::Whitespace
                } else {
                    Token::Illegal
                }
            }

            None => Token::EOF
            }
        }
    }
