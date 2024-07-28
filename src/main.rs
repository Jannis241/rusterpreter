
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


pub mod lexer;
pub mod parser;
pub mod file_reader;

use crate::lexer::*;
use crate::parser::*;

use std::fmt;
use std::process::exit;
use std::env;
use std::fs;

fn main() {
    let input = file_reader::get_input();
    let tokens = Lexer::create_tokens(input.into());

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

    parser.add_rule(vec![TokenName::PRINT, TokenName::LPARENT, TokenName::EXPR, TokenName::RPARENT, TokenName::SEMICOLON], TokenName::STATEMENT);
    parser.add_rule(vec![TokenName::VAR, TokenName::EXPR, TokenName::ASSIGN, TokenName::EXPR, TokenName::SEMICOLON], TokenName::STATEMENT);
    
    let ast = parser.parse();
    for line in ast.clone() {
        line.eval();
    }
    println!("");
    println!("{:?}", ast);
    
}
