use lexer;
use token::Token;
use token;

#[test]
fn test_indent_lookup() {
    assert_eq!(token::indent_lookup("2"), Token::Indent(String::from("2")));
    assert_eq!(token::indent_lookup("/"), Token::Frac);
}

#[test]
fn test_lexer(){
    let mut x = lexer::Lexer::new("a Bad/c");
    let results = vec![Token::Indent("a".to_string()),
                   Token::Whitespace,
                   Token::Indent("Bad".to_string()),
                   Token::Frac,
                   Token::Indent("c".to_string())];
    for i in results {
        assert_eq!(i, x.next_token());
    }
}