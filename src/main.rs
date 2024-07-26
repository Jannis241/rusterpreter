mod lexer;
mod file_reader;


use crate::lexer::Lexer;



fn main() {
                        

    let input = file_reader::get_input();
    let tokens = Lexer::create_tokens(input.into());
    println!("{:?}", tokens)
}
