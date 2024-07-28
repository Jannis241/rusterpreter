// main.rs

mod lexer;
mod parser;
mod file_reader;


use lexer::{Token, TokenName, Lexer};
use parser::Parser;

fn main() {
    let input = file_reader::get_input();
    let tokens = Lexer::create_tokens(input.into());
    println!("tokens: {:?}", tokens);
    let mut parser = Parser::new(tokens);
    parser.add_rule(vec![TokenName::INT], TokenName::EXPR);
    parser.add_rule(vec![TokenName::IDENT], TokenName::EXPR);
    parser.add_rule(vec![TokenName::STRING], TokenName::EXPR);
    parser.add_rule(vec![TokenName::FALSE], TokenName::EXPR);
    parser.add_rule(vec![TokenName::TRUE], TokenName::EXPR);

    parser.decrease_importance();

    parser.add_rule(vec![TokenName::EXPR, TokenName::ASTERICS, TokenName::EXPR], TokenName::EXPR);
    parser.add_rule(vec![TokenName::EXPR, TokenName::SLASH, TokenName::EXPR], TokenName::EXPR);

    parser.decrease_importance();

    parser.add_rule(vec![TokenName::EXPR, TokenName::PLUS, TokenName::EXPR], TokenName::EXPR);
    parser.add_rule(vec![TokenName::EXPR, TokenName::MINUS, TokenName::EXPR], TokenName::EXPR);
    
    parser.decrease_importance();

    parser.add_rule(vec![TokenName::PRINT, TokenName::COLON, TokenName::EXPR], TokenName::STATEMENT);
    parser.add_rule(vec![TokenName::EXPR, TokenName::ASSIGN, TokenName::EXPR], TokenName::STATEMENT);
    
    let ast = parser.parse();
    ast.eval();
    println!("");
    println!("{:?}", ast);
    
}
