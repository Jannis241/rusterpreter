use crate::*;

pub struct Lexer {
    current_char: u8,
    current_position: usize,
    read_position: usize,
    input: Vec<u8>,
}
#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub name: TokenName,
    pub value: String,
}

impl Token {
    fn new(name: TokenName, value: &str) -> Self {
        let value = value.to_string();
        Token {
            name,
            value
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenName {

    // für den parser
    EXPR,
    STATEMENT,
    PARENTS,
    MULTIPLICATION,
    DIVISION,
    ADD,
    SUBTRACT,




    // lexer
    EOF,
    INVALID,

    STRING,
    INT,
    IDENT,

    SEMICOLON,
    COLON,
    COMMA,
    RPARENT,
    LPARENT,
    RCURLY,
    LCURLY,
    LBRACKET,
    RBRACKET,

    MINUS,
    PLUS,
    ASTERICS,
    SLASH,
    DOT,

    IST,
    GLEICH,
    EQUAL,
    ASSIGN,

    NOT_EQUAL,

    LESS_THAN,
    GREATER_THAN,
    BANG,
    HASHTAG,

    LESS_OR_EQUAL,
    GREATER_OR_EQUAL,
    
    VAR,
    FUNC,
    IF,
    ELSE,
    RETURN,
    PRINT,
    WHILE,
    TRUE,
    FALSE,
    ODER,
    UND,

}

impl Lexer {
    pub fn create_tokens(inp: String) -> Vec<Token> {
        let mut lex = Lexer {
            current_char: 0,
            current_position: 0,
            read_position: 0,
            input: inp.into_bytes(),
        };

        lex.read_next_char();
        
        let mut tokenList = Vec::new();
        
        loop {
            let token = lex.create_next_token();
            
            if token.name == TokenName::EOF {
                tokenList.push(token);
                break;
            }
            tokenList.push(token);
        }
        tokenList
    }

    fn create_next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let tok = match self.current_char {
            b';' => Token::new(TokenName::SEMICOLON, ";"),
            b':' => Token::new(TokenName::COLON, ":"),
            b',' => Token::new(TokenName::COMMA, ","),
            b'(' => Token::new(TokenName::LPARENT, "("),
            b')' => Token::new(TokenName::RPARENT, ")"),
            b'{' => Token::new(TokenName::LCURLY, "{"),
            b'}' => Token::new(TokenName::RCURLY, "}"),
            b'[' => Token::new(TokenName::LBRACKET, "["),
            b']' => Token::new(TokenName::RBRACKET, "]"),
            b'-' => Token::new(TokenName::MINUS, "-"),
            b'+' => Token::new(TokenName::PLUS, "+"),
            b'*' => Token::new(TokenName::ASTERICS, "*"),
            b'=' => {
                if self.peek_next_char() == b'=' {
                    self.read_next_char();
                    Token::new(TokenName::EQUAL, "==")

                }
                else {
                    Token::new(TokenName::ASSIGN, "=")
                }
            }
            b'/' => {
                if self.peek_next_char() == b'/' {
                    self.skip_comment();
                    return self.create_next_token();
                } else {
                    Token::new(TokenName::SLASH, "/")
                }
            }
            b'.' => Token::new(TokenName::DOT, "."),
            b'!' => {
                if self.peek_next_char() == b'=' {
                    self.read_next_char();
                    Token::new(TokenName::NOT_EQUAL, "!=")
                } else {
                    Token::new(TokenName::BANG, "!")
                }
            }
            
            b'<' => {
                if self.peek_next_char() == b'=' {
                    self.read_next_char();
                    Token::new(TokenName::LESS_OR_EQUAL, "<=")
                }
                else {
                    Token::new(TokenName::LESS_THAN, "<")
                }
            }            
            b'>' => {
                if self.peek_next_char() == b'=' {
                    self.read_next_char();
                    Token::new(TokenName::GREATER_OR_EQUAL, ">=")
                }
                else {
                    Token::new(TokenName::GREATER_THAN, ">")
                }
            } 
            b'#' => Token::new(TokenName::HASHTAG, "#"),
            b'0'..=b'9' => self.lex_number(),
            b'a' ..= b'z' | b'A' ..= b'Z' | b'_' => self.lex_identifier(),
            b'"' => self.lex_string(),
            0 => Token::new(TokenName::EOF, "eof"),
            _ => {
                println!("not implemented token found: {:?}", self.current_char as char);
                exit(69)
            }
        };       
        self.read_next_char();

        tok
    }

    fn skip_whitespaces(&mut self) {
        while self.current_char.is_ascii_whitespace(){
            self.read_next_char();
        }
    }

    fn peek_next_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_next_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = 0
        } else {
            self.current_char = self.input[self.read_position];
        }
        self.current_position += 1;
        self.read_position += 1;
        
    }

    fn lex_number(&mut self) -> Token {
        let start = self.current_position - 1;
        while self.current_char.is_ascii_digit() || self.current_char == b'.' || self.current_char == b',' {
            self.read_next_char();
        }
        self.current_position -= 1;
        self.read_position -= 1;
        let num_str = String::from_utf8(self.input[start..self.current_position].to_vec()).unwrap();
        Token::new(TokenName::INT, num_str.clone().as_str())
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.current_position - 1;
        while self.current_char.is_ascii_alphanumeric() || self.current_char == b'_' {
            self.read_next_char();
        }

        self.current_position -= 1;
        self.read_position -= 1;

        let ident_str = String::from_utf8(self.input[start..self.current_position].to_vec()).unwrap();

        let tok = match &ident_str[..]{
            "let" => Token::new(TokenName::VAR, "let"),
            "fn" => Token::new(TokenName::FUNC, "fn"),
            "if" => Token::new(TokenName::IF, "if"),
            "else" => Token::new(TokenName::ELSE, "else"),
            "print" => Token::new(TokenName::PRINT, "print"),
            "while" => Token::new(TokenName::WHILE, "while"),
            "return" => Token::new(TokenName::RETURN, "return"),
            "false" => Token::new(TokenName::FALSE, "false"),
            "true" => Token::new(TokenName::TRUE, "true"),
            "and" => Token::new(TokenName::UND, "and"),
            "or" => Token::new(TokenName::ODER, "or"),

            
            _ => Token::new(TokenName::IDENT, ident_str.clone().as_str())
        };
        return tok;
    }
    fn lex_string(&mut self) -> Token {
        let startPos = self.current_position; // Start of the string content

        // Skip the opening quote
        self.read_next_char();

        while self.current_char != b'"' && self.current_char != 0 {
            self.read_next_char();
        }

        if self.current_char == 0 {
            return Token::new(TokenName::INVALID, "invalid"); // Unclosed string literal
        }

        let string_content = String::from_utf8(self.input[startPos..self.current_position - 1].to_vec()).unwrap();
        Token::new(TokenName::STRING, string_content.clone().as_str())
    }
    fn skip_comment(&mut self) {
        while self.current_char != b'\n' && self.current_char != 0 {
            self.read_next_char();
        }
    }
}

