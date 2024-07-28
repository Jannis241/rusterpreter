
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


pub mod lexer;
pub mod parser;
pub mod file_reader;
pub mod ast;

use crate::lexer::*;
use crate::parser::*;
use crate::ast::*;

use std::fmt;
use std::process::exit;
use std::env;
use std::fs;

fn main() {
    let input = file_reader::get_input();
    let tokens = Lexer::create_tokens(input.into());

    let mut parser = Parser::new(tokens);

    // basic types
    parser.add_rule(vec![TokenName::INT], TokenName::EXPR, TokenName::INT);
    parser.add_rule(vec![TokenName::IDENT], TokenName::EXPR, TokenName::IDENT);
    parser.add_rule(vec![TokenName::STRING], TokenName::EXPR, TokenName::STRING);
    parser.add_rule(vec![TokenName::FALSE], TokenName::EXPR, TokenName::FALSE);
    parser.add_rule(vec![TokenName::TRUE], TokenName::EXPR, TokenName::TRUE);

    parser.decrease_importance();

    // print -> die print klammern sind wichtiger als die rechnungs klammern zb 
    parser.add_rule(vec![TokenName::PRINT, TokenName::LPARENT, TokenName::EXPR, TokenName::RPARENT, TokenName::SEMICOLON], TokenName::STATEMENT, TokenName::PRINT);
    
    parser.decrease_importance();

    // Klammern
    parser.add_rule(vec![TokenName::LPARENT, TokenName::EXPR, TokenName::RPARENT], TokenName::EXPR, TokenName::PARENTS);

    parser.decrease_importance();

    // Multiplikation und Division
    parser.add_rule(vec![TokenName::EXPR, TokenName::ASTERICS, TokenName::EXPR], TokenName::EXPR, TokenName::MULTIPLICATION);
    parser.add_rule(vec![TokenName::EXPR, TokenName::SLASH, TokenName::EXPR], TokenName::EXPR, TokenName::DIVISION);

    parser.decrease_importance();

    // Addition und Subtraktion
    parser.add_rule(vec![TokenName::EXPR, TokenName::PLUS, TokenName::EXPR], TokenName::EXPR, TokenName::ADD);
    parser.add_rule(vec![TokenName::EXPR, TokenName::MINUS, TokenName::EXPR], TokenName::EXPR, TokenName::SUBTRACT);

    parser.decrease_importance();

    // variablen definition
    parser.add_rule(vec![TokenName::VAR, TokenName::EXPR, TokenName::ASSIGN, TokenName::EXPR, TokenName::SEMICOLON], TokenName::STATEMENT, TokenName::VAR);
    
    parser.decrease_importance();
    let ast = parser.parse();

    println!("{:?}", ast);

    for line in ast.clone() {
        line.eval();
    }

}

