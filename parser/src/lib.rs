use std::collections::HashMap;
use std::fs;

pub struct Parser {
    file_path: String,
    input_string: String,
    symbol_table: HashMap<String, String>,
}

impl Parser {
    pub fn new(file_path: String) -> Parser {
        Parser {
            file_path,
            input_string: String::from("hi"),
            symbol_table: HashMap::new(),
        }
    }

    pub fn read_file(&mut self) {
        let contents =
            fs::read_to_string(&self.file_path).expect("Should have been able to read the file");

        let lines = contents.lines();

        for line in lines {
            let blank = line.trim().is_empty();
            if !blank {
                println!("{line}");
            }
        }
    }
}
