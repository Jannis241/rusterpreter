
use std::vec;

use crate::lexer::{Lexer, Token};

#[test]
fn comment_test(){
    let input = "// hallo das hier ist ein comment";
    let solution: Vec<Token> = vec![Token::EOF]; 
    test_lexer(input, solution);
}

#[test]
fn comment_string_handling_test(){
    let input = r#"
    variable string = "hallo welt // kommentar"
    schreibe("danach");

    "hallo fufu // schreibe (((((((((((())))))))))))""""""//////....dsafjhsdkfsaf ##### 23483443298423 ___ let returnt wenn

    
    "#;
    let solution: Vec<Token> = vec![
    Token::VAR, Token::IDENT("string".to_string()), Token::ASSIGN, Token::INVALID,
    Token::PRINT, Token::LPARENT, Token::STRING("danach".to_string()), Token::RPARENT, Token::SEMICOLON,
    Token::INVALID, Token::EOF

    ];

    test_lexer(input, solution);

}
#[test]
fn general_test_3() {
    // Test various scenarios including edge cases for keywords, comments, and strings.

    // Test case with keywords in different contexts
    let input = r#"
        variable wahr falsch und oder // comment with keywords
        funktion // function keyword
        wenn x == 10 und y == 20
        schreibe "test string" // another comment
        while wahr // keyword in a while loop
        return und // return keyword followed by und
        // invalid identifier combining false and true
        "unclosed string // unterminated string literal
        // comment with unterminated string "string content
        "#;

    let solution = vec![
        Token::VAR,
        Token::TRUE,
        Token::FALSE,
        Token::UND,
        Token::ODER,
        Token::FUNC,
        Token::IF,
        Token::IDENT("x".to_string()), // assuming x is a valid identifier
        Token::EQUAL,
        Token::INT("10".to_string()),
        Token::UND,
        Token::IDENT("y".to_string()), // assuming y is a valid identifier
        Token::EQUAL,
        Token::INT("20".to_string()),
        Token::PRINT,
        Token::STRING("test string".to_string()),
        Token::WHILE,
        Token::TRUE,
        Token::RETURN,
        Token::UND,
        Token::INVALID, // for "unclosed string
        Token::EOF
    ];

    test_lexer(input, solution); 
}

#[test]
fn test_comments() {
    let input = r#"
        // This is a comment
        variable x = 42; // Comment after code
        // Another comment
        schreibe("Hello, World!"); // Another one
    "#;

    let solution = vec![
        Token::VAR, Token::IDENT("x".to_string()), Token::ASSIGN, Token::INT("42".to_string()), Token::SEMICOLON,
        Token::PRINT, Token::LPARENT, Token::STRING("Hello, World!".to_string()), Token::RPARENT, Token::SEMICOLON,
        Token::EOF
    ];
    test_lexer(input, solution);
}

#[test]
fn test_comment_edge_cases() {
    let input = r#"
        variable x = 10;
        // Comment here
        variable y = 20; // Comment after code
        // Another comment
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
        // End of file comment
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
        Token::EOF
    ];
    test_lexer(input, solution)
}
#[test]
fn comment_test_advanced(){
    let input = r#"
        // das hier ist ein comment
        schreibe("das hier sollte aber wieder gehen");


        1234 // comment
        "neuer string"

        ident // fufu


    "#;
    let solution: Vec<Token> = vec![
        Token::PRINT, Token::LPARENT, Token::STRING("das hier sollte aber wieder gehen".into()), 
        Token::RPARENT, Token::SEMICOLON,
        Token::INT("1234".to_string()),
        Token::STRING("neuer string".to_string()),
        Token::IDENT("ident".to_string()),
        Token::EOF]; 
    test_lexer(input, solution);
}

#[test]
fn basic_chars() {
    let input = ";:,(){}[]-+*/.=!<># //";
    let solution: Vec<Token> = vec![
        Token::SEMICOLON,
        Token::COLON,
        Token::COMMA,
        Token::LPARENT,
        Token::RPARENT,
        Token::LCURLY,
        Token::RCURLY,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::MINUS,
        Token::PLUS,
        Token::ASTERICS,
        Token::SLASH,
        Token::DOT,
        Token::ASSIGN,
        Token::BANG,
        Token::LESS_THAN,
        Token::GREATER_THAN,
        Token::HASHTAG,
        Token::EOF,
    ];
    let mut lexer = Lexer::new(input.into());
    let mut tok_list = Vec::new();
    loop {
        let tok = lexer.create_next_token();
        if tok == Token::EOF{
            tok_list.push(tok);
            break;
        }
        tok_list.push(tok)
    }

    assert_eq!(tok_list.len(), solution.len());
    assert_eq!(tok_list, solution);
}

#[test]
fn double_chars(){
    let input = "== , != , >= , <=";
    let solution: Vec<Token> = vec![
        Token::EQUAL,
        Token::COMMA,
        Token::NOT_EQUAL,
        Token::COMMA,
        Token::GREATER_OR_EQUAL,
        Token::COMMA,
        Token::LESS_OR_EQUAL,
        Token::EOF
    ];
    let mut lexer = Lexer::new(input.into());
    let mut tok_list = Vec::new();
    loop {
        let tok = lexer.create_next_token();
        if tok == Token::EOF{
            tok_list.push(tok);
            break;
        }
        tok_list.push(tok)
    }

    assert_eq!(tok_list.len(), solution.len());
    assert_eq!(tok_list, solution);
}

#[test]
fn handle_empty_input(){
    let input = r#""#;
    let solution = vec![Token::EOF];
    test_lexer(input, solution);
}
#[test]
fn handle_invalid_strings(){
    let input = r#"
    variable fufu = "hallo welt;
    "#;
    let solution = vec![
        Token::VAR,
        Token::IDENT("fufu".into()),
        Token::ASSIGN,
        Token::INVALID,
        Token::EOF
        ];
    test_lexer(input, solution); 
}

#[test]
fn handle_invalid_strings_complicated(){
    let input = r#"
    variable fufu = "hallo welt; 234 q234j ehwfq hsdjkaf akjsdf jsah__ hjahfsd fjahef 235 let var
    "#;
    let solution = vec![
        Token::VAR,
        Token::IDENT("fufu".into()),
        Token::ASSIGN,
        Token::INVALID,
        Token::EOF
        ];
    test_lexer(input, solution); 
}

#[test]
fn semicolon_test(){
    let input = r#"
    wenn;
    1234;
    "hallo welt"; // comment test
    name;
    variable fufu = "cool";
    "#;
    let solution = vec![
        Token::IF,
        Token::SEMICOLON,
        Token::INT("1234".into()),
        Token::SEMICOLON,
        Token::STRING("hallo welt".to_string()),
        Token::SEMICOLON,
        Token::IDENT("name".into()),
        Token::SEMICOLON,
        Token::VAR,
        Token::IDENT("fufu".into()),
        Token::ASSIGN,
        Token::STRING("cool".to_string()),
        Token::SEMICOLON,
        Token::EOF
        ];
    test_lexer(input, solution); 

}




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



    println!("output: {:?}", tokens);
    println!("-----------------------------");
    println!("expected: {:?}", expected_tokens);
    assert_eq!(tokens.len(), expected_tokens.len());
    assert_eq!(tokens, expected_tokens);
}
#[test]
fn test_single_character_tokens() {
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
fn test_two_character_tokens() {
    let input = r#"== != <= >= "#;
    let solution = vec![
        Token::EQUAL, Token::NOT_EQUAL, Token::LESS_OR_EQUAL, Token::GREATER_OR_EQUAL, Token::EOF
    ];
    test_lexer(input, solution);
}

#[test]
fn test_number_tokens() {
    let input = r#"123 456 789"#;
    let solution = vec![
        Token::INT("123".to_string()),
        Token::INT("456".to_string()),
        Token::INT("789".to_string()),
        Token::EOF
    ];
    test_lexer(input, solution);
}

#[test]
fn test_identifier_tokens() {
    let input = r#"variable funktion wenn sonst schreibe while return identifier"#;
    let solution = vec![
        Token::VAR, Token::FUNC, Token::IF, Token::ELSE, Token::PRINT,
        Token::WHILE, Token::RETURN, Token::IDENT("identifier".to_string()),
        Token::EOF
    ];
    test_lexer(input, solution);
}

#[test]
fn test_string_tokens() {
    let input = r#""hello" "world" "string with spaces""#;
    let solution = vec![
        Token::STRING("hello".to_string()),
        Token::STRING("world".to_string()),
        Token::STRING("string with spaces".to_string()),
        Token::EOF
    ];
    test_lexer(input, solution);
}


#[test]
fn test_whitespace_and_newlines() {
    let input = r#"
        // comments am anfang 
        variable    x       = 10;
        variable
        y  = 20;
        wenn(x!=y) {
            // test
            schreibe( " x is not equal to y" );
        }sonst {
            schreibe( " x is equal to y");
        }

        "whitespace test"
        // comments am ende
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
fn test_edge_cases() {
    let input = r#"
        
        variable __a = 001;
        variable b1 = 100;
        variable c = var;
        wenn (__a != b1) {
            schreibe("Test with underscores: __a");
        }
        ;
        variable fufu = "hallo
        // nach invalid kann uahc ein kommentar kommen
    "#;

    let solution = vec![
        Token::VAR, Token::IDENT("__a".to_string()), Token::ASSIGN, Token::INT("001".to_string()), Token::SEMICOLON,
        Token::VAR, Token::IDENT("b1".to_string()), Token::ASSIGN, Token::INT("100".to_string()), Token::SEMICOLON,
        Token::VAR, Token::IDENT("c".to_string()), Token::ASSIGN, Token::IDENT("var".to_string()), Token::SEMICOLON,
        Token::IF, Token::LPARENT, Token::IDENT("__a".to_string()), Token::NOT_EQUAL, Token::IDENT("b1".to_string()), Token::RPARENT,
        Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("Test with underscores: __a".to_string()), Token::RPARENT, Token::SEMICOLON,
        Token::RCURLY,
        Token::SEMICOLON,
        Token::VAR, Token::IDENT("fufu".to_string()), Token::ASSIGN, Token::INVALID,
        Token::EOF
    ];

    test_lexer(input, solution);
}

#[test]
fn test_hard_mixed_tokens() {
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
fn test_mixed_tokens() {
    let input = r#"
        variable x = 42;
        wenn (x < 100) {
            schreibe("x is less than 100");
        } sonst {
            schreibe("x is greater than or equal to 100");
        }
    "#;
    let solution = vec![
        Token::VAR, Token::IDENT("x".to_string()), Token::ASSIGN, Token::INT("42".to_string()),
        Token::SEMICOLON, Token::IF, Token::LPARENT, Token::IDENT("x".to_string()),
        Token::LESS_THAN, Token::INT("100".to_string()), Token::RPARENT,
        Token::LCURLY, Token::PRINT, Token::LPARENT, Token::STRING("x is less than 100".to_string()), Token::RPARENT,
        Token::SEMICOLON, Token::RCURLY, Token::ELSE, Token::LCURLY,
        Token::PRINT, Token::LPARENT, Token::STRING("x is greater than or equal to 100".to_string()), Token::RPARENT,
        Token::SEMICOLON, Token::RCURLY, Token::EOF
    ];
    test_lexer(input, solution);
}


    #[test]
    fn test_single_semicolon() {
        let input = ";";
        let mut lexer = Lexer::new(input.to_string());
        let token = lexer.create_next_token();
        assert_eq!(token, Token::SEMICOLON);
        assert_eq!(lexer.create_next_token(), Token::EOF);
    }

    #[test]
    fn test_semicolon_with_whitespace() {
        let input = "          ;       ";
        let mut lexer = Lexer::new(input.to_string());
        let token = lexer.create_next_token();
        assert_eq!(token, Token::SEMICOLON);
        assert_eq!(lexer.create_next_token(), Token::EOF);
    }

    #[test]
    fn test_semicolon_in_code() {
        let input = "variable x = 5; schreibe(x);";
        
        
        let expected_output = vec![
            Token::VAR,
            Token::IDENT("x".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::PRINT,
            Token::LPARENT,
            Token::IDENT("x".to_string()),
            Token::RPARENT,
            Token::SEMICOLON,
            Token::EOF,
        ];
        test_lexer(input, expected_output);
    }

    #[test]
    fn test_multiple_semicolons() {
        let input = ";;";
        let mut lexer = Lexer::new(input.to_string());

        // Check first semicolon
        let token = lexer.create_next_token();
        assert_eq!(token, Token::SEMICOLON);

        // Check second semicolon
        let token = lexer.create_next_token();
        assert_eq!(token, Token::SEMICOLON);

        // Check end of input
        assert_eq!(lexer.create_next_token(), Token::EOF);
    }
    #[test]
    fn general_semicolon(){
        let input = r#"
            "string";
            1234;
            ident;
            ;;
            "fu ; fu"
        "#;
        let expected = vec![
            Token::STRING("string".to_string()),
            Token::SEMICOLON,
            Token::INT("1234".to_string()),
            Token::SEMICOLON,
            Token::IDENT("ident".to_string()),
            Token::SEMICOLON,
            Token::SEMICOLON,
            Token::SEMICOLON,
            Token::STRING("fu ; fu".to_string()),
            Token::EOF,
        ];


        test_lexer(input, expected);
    }



    #[test]
    fn test_semicolon_with_comment() {
        let input = "x = 5; // this is a comment\n y = 10;";
        let expected_output = vec![
            Token::IDENT("x".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::IDENT("y".to_string()),
            Token::ASSIGN,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::EOF,
        ];
        test_lexer(input, expected_output);
    }

