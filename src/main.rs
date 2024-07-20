use std::{fs::File, iter::Enumerate, vec};


fn get_file_of_name(name: &String) -> Option<File> {
    let file = File::create("./test.txt").expect("error while creating file");
    Some(file)
} 

fn read_file(file: File) -> Vec<Vec<String>> {


    // returnt all lines with all words -> [[word 1, word2], [word1, word2], [word1, word2]]
    let word1 = String::from("fufu");
    let word2 = String::from("dede");
    let word3 = String::from("sleepy");
    let word4 = String::from("bro");
    return vec![vec![word1, word2], vec![word3, word4]];
}

fn main() {
    // man hat zb ein file "fortnite.dd"
    // dann soll man einfach machen: "dede fortnite.dd"
    // dann wird die get_file_of_name function gerufen die dann das file oder none returnt
    // das file wird das gelesen und zu dem lexer geschickt und es in tokens zu verwandeln
    // das ganze wird das geparst und ausgeführt
    let file = get_file_of_name(&"name".to_string());

    if let Some(file) = file { 
        // its not none so a file was found
        let content = read_file(file);
        for line in content {
            println!("line: {:?}", line);
        }
    }

}