use crate::lexer::{Lexer, Token};
#[test]
fn keywords_and_identifiers_test() {
    let input = r#"
        variable funktion wenn sonst schreibe while return
        wahr falsch und oder other
    "#;

    let solution = vec![
        Token::VAR,
        Token::FUNC,
        Token::IF,
        Token::ELSE,
        Token::PRINT,
        Token::WHILE,
        Token::RETURN,
        Token::TRUE,
        Token::FALSE,
        Token::UND,
        Token::ODER,
        Token::IDENT("other".to_string()),
        Token::EOF
    ];
    test_lexer(input, solution);
}

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
    fn test_single_character_tokens() {
        test_lexer(
            ";:,.(){}[]+-*/=<>!",
            vec![
                Token::SEMICOLON, Token::COLON, Token::COMMA, Token::DOT, Token::LPARENT,
                Token::RPARENT, Token::LCURLY, Token::RCURLY, Token::LBRACKET, Token::RBRACKET,
                Token::PLUS, Token::MINUS, Token::ASTERICS, Token::SLASH, Token::ASSIGN,
                Token::LESS_THAN, Token::GREATER_THAN, Token::BANG, Token::EOF
            ]
        );
    }

    #[test]
    fn test_double_character_tokens() {
        test_lexer(
            "== != <= >= //",
            vec![
                Token::EQUAL, Token::NOT_EQUAL, Token::LESS_OR_EQUAL, Token::GREATER_OR_EQUAL, Token::EOF
            ]
        );
    }

    #[test]
    fn test_identifiers_and_keywords() {
        test_lexer(
            "variable funktion wenn sonst schreibe while return var1 func2",
            vec![
                Token::VAR, Token::FUNC, Token::IF, Token::ELSE, Token::PRINT, Token::WHILE,
                Token::RETURN, Token::IDENT("var1".to_string()), Token::IDENT("func2".to_string()), Token::EOF
            ]
        );
    }

    #[test]
    fn test_numbers() {
        test_lexer(
            "123 456 7890",
            vec![
                Token::INT("123".to_string()), Token::INT("456".to_string()), Token::INT("7890".to_string()), Token::EOF
            ]
        );
    }

    #[test]
    fn test_strings() {
        test_lexer(
            r#""hello" "world" "123""#,
            vec![
                Token::STRING("hello".to_string()), Token::STRING("world".to_string()), Token::STRING("123".to_string()), Token::EOF
            ]
        );
    }

    #[test]
    fn test_whitespace() {
        test_lexer(
            " \t\n\r",
            vec![Token::EOF]
        );
    }

    #[test]
    fn test_comments() {
        test_lexer(
            "// this is a comment\nvariable",
            vec![Token::VAR, Token::EOF]
        );

        test_lexer(
            "// comment with special characters !@#$%^&*()_+",
            vec![Token::EOF]
        );

        test_lexer(
            "//\nvariable // another comment\nfunktion",
            vec![Token::VAR, Token::FUNC, Token::EOF]
        );
    }

    #[test]
    fn test_unclosed_string() {
        test_lexer(
            r#""unclosed string"#,
            vec![Token::INVALID, Token::EOF]
        );
    }

    #[test]
    fn test_string_with_comment() {
        test_lexer(
            r#""string // comment""#,
            vec![Token::INVALID, Token::EOF]
        );
    }

    #[test]
    fn test_mixed_input() {
        test_lexer(
            r#"variable x = 10;
            funktion foo() {
                wenn (x > 10) {
                    schreibe "x is greater than 10";
                } sonst {
                    schreibe "x is not greater than 10";
                }
                return x;
            }"#,
            vec![
                Token::VAR, Token::IDENT("x".to_string()), Token::ASSIGN, Token::INT("10".to_string()), Token::SEMICOLON,
                Token::FUNC, Token::IDENT("foo".to_string()), Token::LPARENT, Token::RPARENT, Token::LCURLY,
                Token::IF, Token::LPARENT, Token::IDENT("x".to_string()), Token::GREATER_THAN, Token::INT("10".to_string()), Token::RPARENT, Token::LCURLY,
                Token::PRINT, Token::STRING("x is greater than 10".to_string()), Token::SEMICOLON, Token::RCURLY,
                Token::ELSE, Token::LCURLY,
                Token::PRINT, Token::STRING("x is not greater than 10".to_string()), Token::SEMICOLON, Token::RCURLY,
                Token::RETURN, Token::IDENT("x".to_string()), Token::SEMICOLON,
                Token::RCURLY, Token::EOF
            ]
        );
    }

    #[test]
    fn test_edge_cases() {
        // Edge case: mixing valid and invalid tokens
        test_lexer(
            r#"variable x = 10; # 
            funktion foo() {
                wenn (x > 10) {
                    schreibe "x is greater than 10";
                } sonst {
                    schreibe "x is not greater than 10";
                }
                return x;
            }"#,
            vec![
                Token::VAR, Token::IDENT("x".to_string()), Token::ASSIGN, Token::INT("10".to_string()), Token::SEMICOLON, Token::HASHTAG,
                Token::FUNC, Token::IDENT("foo".to_string()), Token::LPARENT, Token::RPARENT, Token::LCURLY,
                Token::IF, Token::LPARENT, Token::IDENT("x".to_string()), Token::GREATER_THAN, Token::INT("10".to_string()), Token::RPARENT, Token::LCURLY,
                Token::PRINT, Token::STRING("x is greater than 10".to_string()), Token::SEMICOLON, Token::RCURLY,
                Token::ELSE, Token::LCURLY,
                Token::PRINT, Token::STRING("x is not greater than 10".to_string()), Token::SEMICOLON, Token::RCURLY,
                Token::RETURN, Token::IDENT("x".to_string()), Token::SEMICOLON,
                Token::RCURLY, Token::EOF
            ]
        );
    }

