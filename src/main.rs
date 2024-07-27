mod lexer;
mod parser;
mod file_reader;
mod ast;

use lexer::{Token, TokenName};

use crate::lexer::Lexer;
use crate::parser::Parser;
fn string_eval (){
    println!("string ist cool");
}

fn ident_eval (){
    println!("ident ist cool");
}
fn int_eval (){
    println!("int ist cool");
}
fn main() {
    let input = file_reader::get_input();
    
    let tokens = Lexer::create_tokens(input.into());
    
    let mut parser = Parser::new(tokens);
    parser.add_rule(vec![TokenName::IDENT], TokenName::EXPR);
    parser.add_rule(vec![TokenName::STRING], TokenName::EXPR);
    parser.add_rule(vec![TokenName::INT], TokenName::EXPR);
    parser.add_rule(vec![TokenName::FALSE], TokenName::EXPR);
    parser.add_rule(vec![TokenName::TRUE], TokenName::EXPR);
 
    parser.decrease_importance();
    
    parser.add_rule(vec![TokenName::EXPR, TokenName::ASTERICS, TokenName::EXPR], TokenName::EXPR);
    parser.add_rule(vec![TokenName::EXPR, TokenName::SLASH, TokenName::EXPR], TokenName::EXPR);
    
    parser.decrease_importance();
    
    parser.add_rule(vec![TokenName::EXPR, TokenName::PLUS, TokenName::EXPR], TokenName::EXPR);
    parser.add_rule(vec![TokenName::EXPR, TokenName::MINUS, TokenName::EXPR], TokenName::EXPR);

    let ast = parser.parse();
    ast.eval();
}
