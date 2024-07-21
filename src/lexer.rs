use std::time::Instant;

use crate::*;

#[derive(Debug)]
pub enum Token {
    STRING(String),
    NUM(f64),
    PLUS(String),
    MINUS(String),
    MAL(String),
    GETEILT(String),
    VARIABLE(String),
    LPARENT(String),
    RPARENT(String),
    LCURLY(String),
    RCURLY(String),
    LBRACKET(String),
    RBRACKET(String),
    DOPPELPUNKT(String),
    KOMMA(String),
    PUNKT(String),
    PRINT(String),
    GLEICH(String),

}

fn is_number(content: &String) -> Option<f64>{
    
    match content.parse::<f64>() {
        Ok(_) => Some(content.parse::<f64>().unwrap_or(0.0)),
        Err(_) => None,
    }
}

pub fn adjust_minus_tokens(token_list: &mut Vec<Token>) {
    let mut i = 0;
    let mut indices_to_remove = Vec::new();
    let mut adjustments = Vec::new();

    // First pass: Collect information about which tokens to modify or remove
    while i < token_list.len() {        
        if let Token::MINUS(_) = token_list.get(i).unwrap() {
            if let Some(next_token) = token_list.get(i + 1) {
                if let Token::NUM(number) = next_token {
                    if i == 0 {
                        // The MINUS token is the first token
                        indices_to_remove.push(i);
                        adjustments.push((i, -number)); // Store index of NUM and its new value
                    }
                    else if let Some(previous_token) = token_list.get(i - 1) {
                        if let Token::NUM(m) = previous_token {

                        }
                        else {
                            indices_to_remove.push(i);
                            adjustments.push((i, -number)); // Store index of NUM and its new value 
                        }
                    } 
                }
            }
        }

        i += 1;
    }

    for &index in indices_to_remove.iter().rev() {
        token_list.remove(index);
    }

    for (num_index, new_value) in adjustments {
        if let Some(token) = token_list.get_mut(num_index) {
            *token = Token::NUM(new_value);
        }
    }
}

pub fn create_tokens (line: &String) -> Vec<Token>{
    let mut token_list = Vec::new();
    let mut i = 0;

    while i < line.len(){
        if line.starts_with("//"){
            break // a comment has started so we can ignore the rest of the line
        }

        let current_char = line.chars().nth(i);

        // einzelne char
        if let Some(current_char) = current_char {
            
            // variablen
            if current_char.is_alphabetic() || current_char == '_'{
                let mut text = String::new();
                
                loop {
                    if let Some(c) = line.chars().nth(i){

                        if c.is_alphanumeric() || c == '_' && c != ' '{
                            text.push(c);
                            i += 1;
                        }
                        else {
                            // var has ended
                            break
                        }
                        
                    }
                    else {
                        // end of line
                        break 
                    }
                }

                if text == String::from("schreibe"){
                    token_list.push(Token::PRINT(String::from("schreibe")))

                }
                else {
                    token_list.push(Token::VARIABLE(String::from(text)))

                }

                i -= 1; // minus 1 weil man hat ja noch einmal +1 gerechnet, aber dann wurden die chars unterbrochen, also
                // muss man nochmal einen zurück gehen um an das richtige ende zu kommen da sonst der nächste char nach einer var
                // oder einem string übersprungen wird
            }

            // strings
            if current_char == '"'{
                let mut string = String::new();
                i += 1; // go to the char after the "
                loop {
                    if let Some(c) = line.chars().nth(i){
                        if c == '"'{
                            break // found the end of the string
                        }
                        else {
                            string.push(c);
                            i +=1 ;
                        }
                    }   

                    else{
                        break
                    }
 
                }
                token_list.push(Token::STRING(string));
            }

            // nums

            if let Some(number) = is_number(&current_char.to_string()){
                let mut num = String::new();
                loop {
                    if let Some(c) = line.chars().nth(i){ 
                        if let Some(n) = is_number(&c.to_string()){
                            let n = n.to_string();
                            num.push(n.chars().nth(0).unwrap_or('e'));
                            i+=1;
                        }
                        else {
                            break
                        }
                    }
                    else {
                        break
                    }
                }
                let num = is_number(&num);
                if let Some(n) = num {
                    token_list.push(Token::NUM(n));
                }
                i-= 1
            }

           

            match current_char {
                '=' => {
                    token_list.push(Token::GLEICH(String::from("=")));
                }
                '+' => {
                    token_list.push(Token::PLUS(String::from("+")))
                }
                '-' => {
                    token_list.push(Token::MINUS(String::from("-")))
                }
                '/' => {
                    token_list.push(Token::GETEILT(String::from("/")))
                }
                '*' => {
                    token_list.push(Token::MAL(String::from("*")))
                }
                '(' => {

                    token_list.push(Token::LPARENT(String::from("(")))
                }
                ')' => {

                    token_list.push(Token::RPARENT(String::from(")")))
                }
                '{' => {
                    token_list.push(Token::LCURLY(String::from("{")))

                }
                '}' => {
                    token_list.push(Token::RCURLY(String::from("}")))

                }
                '[' => {
                    token_list.push(Token::LBRACKET(String::from("[")))

                }
                ']' => {
                    token_list.push(Token::RBRACKET(String::from("]")))

                }
                '.' => {
                    token_list.push(Token::PUNKT(String::from(".")))

                }
                ',' => {
                    token_list.push(Token::KOMMA(String::from(",")))

                }
                ':' => {
                    token_list.push(Token::DOPPELPUNKT(String::from(":")))

                }
                _ => {}
            }
        }
        // go to the next char
        i += 1;



    }

    adjust_minus_tokens(&mut token_list);
    return token_list;


}

