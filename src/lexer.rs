use std::process::exit;

pub struct Lexer {
    current_char: u8,
    current_position: usize,
    read_position: usize,
    input: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    INVALID,

    STRING(String),
    INT(String),
    IDENT(String),

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
            
            if token == Token::EOF {
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
            b';' => Token::SEMICOLON,
            b':' => Token::COLON,
            b',' => Token::COMMA,
            b'(' => Token::LPARENT,
            b')' => Token::RPARENT,
            b'{' => Token::LCURLY,
            b'}' => Token::RCURLY,
            b'[' => Token::LBRACKET,
            b']' => Token::RBRACKET,
            b'-' => Token::MINUS,
            b'+' => Token::PLUS,
            b'*' => Token::ASTERICS,
            b'/' => {
                if self.peek_next_char() == b'/' {
                    self.skip_comment();
                    return self.create_next_token();
                } else {
                    Token::SLASH
                }
            }
            b'.' => Token::DOT,
            b'!' => {
                if self.peek_next_char() == b'=' {
                    self.read_next_char();
                    Token::NOT_EQUAL
                } else {
                    Token::BANG
                }
            }
            
            b'<' => {
                if self.peek_next_char() == b'=' {
                    self.read_next_char();
                    Token::LESS_OR_EQUAL
                }
                else {
                    Token::LESS_THAN
                }
            }            
            b'>' => {
                if self.peek_next_char() == b'=' {
                    self.read_next_char();
                    Token::GREATER_OR_EQUAL
                }
                else {
                    Token::GREATER_THAN
                }
            } 
            b'#' => Token::HASHTAG,
            b'0'..=b'9' => self.lex_number(),
            b'a' ..= b'z' | b'A' ..= b'Z' | b'_' => self.lex_identifier(),
            b'"' => self.lex_string(),
            0 => Token::EOF,
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
        while self.current_char.is_ascii_digit() {
            self.read_next_char();
        }
        self.current_position -= 1;
        self.read_position -= 1;
        let num_str = String::from_utf8(self.input[start..self.current_position].to_vec()).unwrap();
        Token::INT(num_str)
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
            "setze" => Token::VAR,
            "funktion" => Token::FUNC,
            "wenn" => Token::IF,
            "sonst" => Token::ELSE,
            "schreibe" => Token::PRINT,
            "while" =>Token::WHILE, 
            "return" => Token::RETURN,
            "falsch" => Token::FALSE,
            "wahr" => Token::TRUE,
            "und" => Token::UND,
            "oder" => Token::ODER,
            "auf" => Token::ASSIGN,
            "gleich" => Token::GLEICH,
            "ist" => Token::IST,
            
            _ => Token::IDENT(ident_str)
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
            return Token::INVALID; // Unclosed string literal
        }

        let string_content = String::from_utf8(self.input[startPos..self.current_position - 1].to_vec()).unwrap();
        Token::STRING(string_content)
    }
    fn skip_comment(&mut self) {
        while self.current_char != b'\n' && self.current_char != 0 {
            self.read_next_char();
        }
    }
}

