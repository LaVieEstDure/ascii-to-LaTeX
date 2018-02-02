use lexer;
use token::Token;
use token;
use ::ast;
use parser::Expression;

#[test]
fn test_indent_lookup() {
    assert_eq!(token::indent_lookup("2"), Token::Indent(String::from("2")));
    assert_eq!(token::indent_lookup("SADlj"), Token::Indent(String::from("SADlj")));
    assert_eq!(token::indent_lookup("sin"), Token::Sin);
    assert_eq!(token::indent_lookup("sqrt"), Token::Sqrt);
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

#[test]
fn test_parser(){
    let input = "sin(cos(a))";
    assert_eq!(ast(input),
               Expression::Unary(
                    Token::Sin, 
                    Box::new(
                        Expression::Unary(
                            Token::Cos, 
                            Box::new(Expression::Indent("a".to_string()))
                        )
                    )
                )
               );
}
