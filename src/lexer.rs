use token;
use token::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    text: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::EOF => None,
            c => Some(c)
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {text: input.chars().peekable()}
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.text.next()
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.text.peek()
    }

    pub fn next_is_letter(&mut self) -> bool {
        match self.peek() {
            Some(&c) => c.is_alphabetic(),
            None => false
        }
    }

    pub fn next_is_numeric(&mut self) -> bool {
        match self.peek() {
            Some(&'.') => true,
            Some(&c) => c.is_numeric(),
            None => false
        }
    }

    pub fn read_identifier(&mut self, first_char: char) -> String {
        let mut identifier = String::new();
        identifier.push(first_char);
        while self.next_is_letter() {
            identifier.push(self.next_char().unwrap());
        }
        identifier
    }

    pub fn read_number(&mut self, first_char: char) -> String {
        let mut identifier = String::new();
        identifier.push(first_char);
        while self.next_is_numeric() {
            identifier.push(self.next_char().unwrap());
        }
        identifier
    }

    pub fn next_token(&mut self) -> Token {
        match self.next_char() {
            Some('/') => Token::Frac,
            Some('+') => Token::Plus,
            Some('(') => Token::LeftParenth,
            Some(')') => Token::RightParenth,
            Some(c) => {
                if c.is_alphabetic() {
                    let identifier = self.read_identifier(c);
                    token::indent_lookup(&identifier)
                } else if c.is_whitespace() {
                    Token::Whitespace
                } else if c.is_numeric() {
                    let identifier = self.read_number(c);
                    Token::Number(identifier)
                } else {
                    Token::Illegal
                }
            }

            None => Token::EOF
            }
        }
    }
