use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::fs::File;
use std::fs;
use std::path::Path;
mod parser_file;
mod lexer;

#[derive(Debug)]
pub enum SearchError {
    NotFound,
    MultipleFound,
    IoError(io::Error),
}
impl From<io::Error> for SearchError {
    fn from(error: io::Error) -> Self {
        SearchError::IoError(error)
    }
}

pub fn find_file_in_path(path: &str, name: &str) -> Result<String, SearchError> {
    let mut found_files = vec![];
    println!("searching in {} for file {}", path, name);
    fn search_directory(path: &PathBuf, name: &str, found_files: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if entry_path.file_name() == Some(std::ffi::OsStr::new(name)) {
                    found_files.push(entry_path.clone());
                }
                search_directory(&entry_path, name, found_files)?;
            }
        }
        Ok(())
    }

    let path = PathBuf::from(path);
    search_directory(&path, name, &mut found_files)?;

    match found_files.len() {
        0 => Err(SearchError::NotFound),
        1 => Ok(found_files.pop().unwrap().to_string_lossy().into_owned()),
        _ => Err(SearchError::MultipleFound),
    }
}

fn get_file_of_name(name: &String) -> Option<File> {

    let path = find_file_in_path("/Users/maus/programmieren/rust/rusterpreter", name);
    
    match path {
        Err(error) => {
            println!("error while searching file '{}' in path", name);
            None
        }
        Ok(path) =>     {
            let file = File::open(path);
            match file {
                Ok(file) => Some(file),
                Err(err) => None,
            }
        }


    }


} 

fn read_file(file: File) -> Result<Vec<String>, io::Error> {
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}
fn main() -> io::Result<()>{
    // man hat zb ein file "fortnite.dd"
    // dann soll man einfach machen: "dede fortnite.dd"
    // dann wird die get_file_of_name function gerufen die dann das file oder none returnt
    // das file wird das gelesen und zu dem lexer geschickt und es in tokens zu verwandeln
    // das ganze wird das geparst und ausgeführt
    let file_name = "main.dd"; // Der Name der Datei, die geöffnet werden soll
    let path = Path::new(file_name);

    // Datei öffnen
    let file = File::open(&path)?;

    // Dateiinhalt zeilenweise lesen
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.len() != 0 {
            lines.push(line);
        }
    }
    for line in lines {
        let tokens = lexer::create_tokens(&line);
        println!("Line: '{}' | Tokens created: {:?}", &line, tokens)
    }
    Ok(())
}

