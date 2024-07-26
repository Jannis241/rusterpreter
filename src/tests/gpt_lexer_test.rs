use crate::lexer::{Lexer, Token};
    // Helper function to test lexer
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

        println!("Output: {:?}", tokens);
        println!("Expected: {:?}", expected_tokens);
        assert_eq!(tokens.len(), expected_tokens.len());
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn gpt_basic_chars() {
        let input = ";:,(){}[]-+*/.=!<>#";
        let solution: Vec<Token> = vec![
            Token::SEMICOLON, Token::COLON, Token::COMMA,
            Token::LPARENT, Token::RPARENT, Token::LCURLY, Token::RCURLY,
            Token::LBRACKET, Token::RBRACKET, Token::MINUS, Token::PLUS,
            Token::ASTERICS, Token::SLASH, Token::DOT, Token::ASSIGN,
            Token::BANG, Token::LESS_THAN, Token::GREATER_THAN,
            Token::HASHTAG, Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_double_chars() {
        let input = "== , != , >= , <=";
        let solution: Vec<Token> = vec![
            Token::EQUAL, Token::COMMA, Token::NOT_EQUAL,
            Token::COMMA, Token::GREATER_OR_EQUAL, Token::COMMA,
            Token::LESS_OR_EQUAL, Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_handle_empty_input() {
        let input = "";
        let solution = vec![Token::EOF];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_handle_invalid_strings() {
        let input = r#"
        variable fufu = "hallo welt;
        "#;
        let solution = vec![
            Token::VAR, Token::IDENT("fufu".into()), Token::ASSIGN,
            Token::INVALID, Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_handle_invalid_strings_complicated() {
        let input = r#"
        variable fufu = "hallo welt; 234 q234j ehwfq hsdjkaf akjsdf jsah__ hjahfsd fjahef 235 let var
        "#;
        let solution = vec![
            Token::VAR, Token::IDENT("fufu".into()), Token::ASSIGN,
            Token::INVALID, Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_semicolon_test() {
        let input = r#"
        wenn;
        1234;
        "hallo welt";
        name;
        variable fufu = "cool";
        "#;
        let solution = vec![
            Token::IF, Token::SEMICOLON, Token::INT("1234".into()), Token::SEMICOLON,
            Token::STRING("hallo welt".to_string()), Token::SEMICOLON,
            Token::IDENT("name".into()), Token::SEMICOLON,
            Token::VAR, Token::IDENT("fufu".into()), Token::ASSIGN,
            Token::STRING("cool".to_string()), Token::SEMICOLON,
            Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_single_character_tokens() {
        let input = r#"; : , ( ) { } [ ] - + * / . = != < > ! #"#;
        let solution = vec![
            Token::SEMICOLON, Token::COLON, Token::COMMA,
            Token::LPARENT, Token::RPARENT, Token::LCURLY, Token::RCURLY,
            Token::LBRACKET, Token::RBRACKET, Token::MINUS, Token::PLUS,
            Token::ASTERICS, Token::SLASH, Token::DOT, Token::ASSIGN,
            Token::NOT_EQUAL, Token::LESS_THAN, Token::GREATER_THAN,
            Token::BANG, Token::HASHTAG, Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_two_character_tokens() {
        let input = r#"== != <= >= "#;
        let solution = vec![
            Token::EQUAL, Token::NOT_EQUAL, Token::LESS_OR_EQUAL, Token::GREATER_OR_EQUAL, Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_number_tokens() {
        let input = r#"123 456 789"#;
        let solution = vec![
            Token::INT("123".to_string()), Token::INT("456".to_string()),
            Token::INT("789".to_string()), Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_identifier_tokens() {
        let input = r#"variable funktion wenn sonst schreibe while return identifier"#;
        let solution = vec![
            Token::VAR, Token::FUNC, Token::IF, Token::ELSE, Token::PRINT,
            Token::WHILE, Token::RETURN, Token::IDENT("identifier".to_string()),
            Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_string_tokens() {
        let input = r#""hello" "world" "string with spaces""#;
        let solution = vec![
            Token::STRING("hello".to_string()), Token::STRING("world".to_string()),
            Token::STRING("string with spaces".to_string()), Token::EOF
        ];
        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_whitespace_and_newlines() {
        let input = r#"
            // kommentareeeee
            variable    x       = 10;
            variable
            y  = 20;
            wenn(x!=y) {
                schreibe( " x is not equal to y" );
            }sonst {
                schreibe( " x is equal to y");
            }

            "whitespace test"
        "#;

        let solution = vec![
            Token::VAR, Token::IDENT("x".to_string()), Token::ASSIGN, Token::INT("10".to_string()), Token::SEMICOLON,
            Token::VAR, Token::IDENT("y".to_string()), Token::ASSIGN, Token::INT("20".to_string()), Token::SEMICOLON,
            Token::IF, Token::LPARENT, Token::IDENT("x".to_string()), Token::NOT_EQUAL, Token::IDENT("y".to_string()), Token::RPARENT,
            Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING(" x is not equal to y".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::ELSE, Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING(" x is equal to y".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::STRING("whitespace test".to_string()), Token::EOF
        ];

        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_edge_cases() {
        let input = r#"
            variable __a = 001;
            variable b1 = 100;
            variable c = var;
            wenn (__a != b1) {
                schreibe("Test with underscores: __a");
            }
            ;
            variable fufu = "hallo
        "#;

        let solution = vec![
            Token::VAR, Token::IDENT("__a".to_string()), Token::ASSIGN, Token::INT("001".to_string()), Token::SEMICOLON,
            Token::VAR, Token::IDENT("b1".to_string()), Token::ASSIGN, Token::INT("100".to_string()), Token::SEMICOLON,
            Token::VAR, Token::IDENT("c".to_string()), Token::ASSIGN, Token::IDENT("var".to_string()), Token::SEMICOLON,
            Token::IF, Token::LPARENT, Token::IDENT("__a".to_string()), Token::NOT_EQUAL, Token::IDENT("b1".to_string()), Token::RPARENT,
            Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("Test with underscores: __a".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::SEMICOLON, Token::VAR, Token::IDENT("fufu".to_string()), Token::ASSIGN, Token::INVALID,
            Token::EOF
        ];

        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_hard_mixed_tokens() {
        let input = r#"
            variable x = 10;
            variable y = 20;
            wenn (x != y) {
                schreibe("x is not equal to y");
            } sonst wenn (x < y) {
                schreibe("x is less than y");
            } sonst {
                schreibe("x is greater than or equal to y");
            }

            x = x + 5;
            y = y - 3;
            schreibe("The final values are: ", x, y);

            "this is a string with special characters: !@#$%^&*()"
            "unclosed string literal
        "#;

        let solution = vec![
            Token::VAR, Token::IDENT("x".to_string()), Token::ASSIGN, Token::INT("10".to_string()), Token::SEMICOLON,
            Token::VAR, Token::IDENT("y".to_string()), Token::ASSIGN, Token::INT("20".to_string()), Token::SEMICOLON,
            Token::IF, Token::LPARENT, Token::IDENT("x".to_string()), Token::NOT_EQUAL, Token::IDENT("y".to_string()), Token::RPARENT,
            Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("x is not equal to y".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::ELSE, Token::IF, Token::LPARENT, Token::IDENT("x".to_string()), Token::LESS_THAN, Token::IDENT("y".to_string()), Token::RPARENT,
            Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("x is less than y".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::ELSE, Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("x is greater than or equal to y".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::IDENT("x".to_string()), Token::ASSIGN, Token::IDENT("x".to_string()), Token::PLUS, Token::INT("5".to_string()), Token::SEMICOLON,
            Token::IDENT("y".to_string()), Token::ASSIGN, Token::IDENT("y".to_string()), Token::MINUS, Token::INT("3".to_string()), Token::SEMICOLON,
            Token::PRINT, Token::LPARENT, Token::STRING("The final values are: ".to_string()), Token::COMMA, Token::IDENT("x".to_string()), Token::COMMA, Token::IDENT("y".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::STRING("this is a string with special characters: !@#$%^&*()".to_string()),
            Token::INVALID, // For the unclosed string literal
            Token::EOF
        ];

        test_lexer(input, solution);
    }

    #[test]
    fn gpt_test_mixed_tokens() {
        let input = r#"
            variable x = 42;
            wenn (x < 100) {
                schreibe("x is less than 100");
            } sonst {
                schreibe("x is greater than or equal to 100");
            }
        "#;
        let solution = vec![
            Token::VAR, Token::IDENT("x".to_string()), Token::ASSIGN, Token::INT("42".to_string()), Token::SEMICOLON,
            Token::IF, Token::LPARENT, Token::IDENT("x".to_string()), Token::LESS_THAN, Token::INT("100".to_string()), Token::RPARENT,
            Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("x is less than 100".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::ELSE, Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("x is greater than or equal to 100".to_string()), Token::RPARENT, Token::SEMICOLON,
            Token::RCURLY, Token::EOF
        ];
        test_lexer(input, solution);
    }
