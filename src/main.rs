use lexer::Token;

#[cfg(test)]

mod tests;
mod lexer;


fn main() {
    let input = r#"

    funktion add(x, y) {
        return x + y
    }

    variable ergebnis = add(x + y);
    schreibe("das hier ist das ergebnise: ", ergebnis);


    wenn ergebnis == x + y {
        schreibe("das ergebnis stimmt!");
    }
    sonst {
        schreibe("der computer hat sich verrechnet, das richtige ergebnis ist: ", x + y);
    }
    let fufu "hallo//"
    let fu// = hallo
    }
    
    "#;
    let mut lexer = lexer::Lexer::new(input.into());

    while let tok = lexer.create_next_token() {
        println!("{:?}", tok);
        if tok == Token::EOF {
            break;
        }
    }
}
