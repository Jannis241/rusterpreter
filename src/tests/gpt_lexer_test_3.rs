use crate::lexer::{Lexer, Token};
fn test_lexer(input: &str, expected_tokens: Vec<Token>) {
    let mut lexer = Lexer::new(input.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.create_next_token();
        if tok == Token::EOF {
            tokens.push(tok);
            break;
        }
        tokens.push(tok);
    }



    println!("output: {:?}", tokens);
    println!("-----------------------------");
    println!("expected: {:?}", expected_tokens);
    assert_eq!(tokens.len(), expected_tokens.len());
    assert_eq!(tokens, expected_tokens);
}
    #[test]
    fn test_basic_tokens() {
        let input = "; : , ( ) { } [ ] - + * / . = == != < > <= >= #";
        let expected = vec![
            Token::SEMICOLON, Token::COLON, Token::COMMA, Token::LPARENT, Token::RPARENT,
            Token::LCURLY, Token::RCURLY, Token::LBRACKET, Token::RBRACKET, Token::MINUS,
            Token::PLUS, Token::ASTERICS, Token::SLASH, Token::DOT, Token::ASSIGN,
            Token::EQUAL, Token::NOT_EQUAL, Token::LESS_THAN, Token::GREATER_THAN,
            Token::LESS_OR_EQUAL, Token::GREATER_OR_EQUAL, Token::HASHTAG, Token::EOF
        ];
        test_lexer(input, expected);
    }

    #[test]
    fn test_string_literals() {
        let input = r#""hello" "world" "unterminated"#;
        let expected = vec![
            Token::STRING("hello".to_string()), Token::STRING("world".to_string()),
            Token::INVALID, Token::EOF
        ];
        test_lexer(input, expected);
    }

    #[test]
    fn test_numbers() {
        let input = "123 0 4567";
        let expected = vec![
            Token::INT("123".to_string()), Token::INT("0".to_string()), Token::INT("4567".to_string()),
            Token::EOF
        ];
        test_lexer(input, expected);
    }

    #[test]
    fn test_identifiers() {
        let input = "variable funktion wenn sonst schreibe while return falsch wahr und oder";
        let expected = vec![
            Token::VAR, Token::FUNC, Token::IF, Token::ELSE, Token::PRINT,
            Token::WHILE, Token::RETURN, Token::FALSE, Token::TRUE, Token::UND, Token::ODER,
            Token::EOF
        ];
        test_lexer(input, expected);
    }

    #[test]
    fn test_identifiers_with_invalid() {
        let input = "var var123 _var123 invalid";
        let expected = vec![
            Token::IDENT("var".to_string()), Token::IDENT("var123".to_string()), 
            Token::IDENT("_var123".to_string()), Token::IDENT("invalid".to_string()),
            Token::EOF
        ];
        test_lexer(input, expected);
    }

    #[test]
    fn test_comments() {
        let input = "var // this is a comment\nvar2";
        let expected = vec![
            Token::IDENT("var".to_string()), Token::IDENT("var2".to_string()), Token::EOF
        ];
        test_lexer(input, expected);
    }





