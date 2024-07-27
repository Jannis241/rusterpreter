use std::{env, process::exit};
use std::fs::{self, read};




fn get_file_name() -> String{
    let mut file_name: Vec<String> = env::args().collect();
    file_name.remove(0);

    file_name.get(0).unwrap_or(&"main".to_string()).clone()

}

fn read_file(filename: String) -> String{
    let content = fs::read_to_string(filename);

    match content {
        Ok(content) => content,
        Err(e) => {
            println!("DD: file not found");
            exit(69)
        }
    }
}


pub fn get_input() -> String {
    let file_name = get_file_name();

    let file_name = if file_name.ends_with(".dd") {
        file_name.to_string()
    } else {
        format!("{}.dd", file_name)
    };
    let content = read_file(file_name);

    return content;
}